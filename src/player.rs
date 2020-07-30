use super::{
    CombatStats, GameLog, Item, Map, Player, Position, RunState, State, Viewshed, WantsToMelee,
    WantsToPickupItem,
};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entities = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in
        (&entities, &players, &mut positions, &mut viewsheds).join()
    {
        if pos.x + delta_x < 1
            || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1
            || pos.y + delta_y > map.height - 1
        {
            return;
        }
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }

        if !map.blocked[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement

    let input_global = &rltk::INPUT;
    let input_mutex = input_global.lock();

    let shift_down = input_mutex.is_key_pressed(VirtualKeyCode::LShift)
        | input_mutex.is_key_pressed(VirtualKeyCode::RShift);
    let control_down = input_mutex.is_key_pressed(VirtualKeyCode::LControl)
        | input_mutex.is_key_pressed(VirtualKeyCode::RControl);

    match ctx.key {
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match (key, shift_down, control_down) {
            // Diagonals. Checked first because of control and shift modifiers
            (VirtualKeyCode::Right, true, false)
            | (VirtualKeyCode::Numpad9, ..)
            | (VirtualKeyCode::U, ..) => try_move_player(1, -1, &mut gs.ecs),

            (VirtualKeyCode::Left, true, false)
            | (VirtualKeyCode::Numpad7, ..)
            | (VirtualKeyCode::Y, ..) => try_move_player(-1, -1, &mut gs.ecs),

            (VirtualKeyCode::Right, false, true)
            | (VirtualKeyCode::Numpad3, ..)
            | (VirtualKeyCode::N, ..) => try_move_player(1, 1, &mut gs.ecs),

            (VirtualKeyCode::Left, false, true)
            | (VirtualKeyCode::Numpad1, ..)
            | (VirtualKeyCode::B, ..) => try_move_player(-1, 1, &mut gs.ecs),

            // Cardinal directions
            (VirtualKeyCode::Left, ..)
            | (VirtualKeyCode::Numpad4, ..)
            | (VirtualKeyCode::H, ..) => try_move_player(-1, 0, &mut gs.ecs),

            (VirtualKeyCode::Right, ..)
            | (VirtualKeyCode::Numpad6, ..)
            | (VirtualKeyCode::L, ..) => try_move_player(1, 0, &mut gs.ecs),

            (VirtualKeyCode::Up, ..) | (VirtualKeyCode::Numpad8, ..) | (VirtualKeyCode::K, ..) => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            (VirtualKeyCode::Down, ..)
            | (VirtualKeyCode::Numpad2, ..)
            | (VirtualKeyCode::J, ..) => try_move_player(0, 1, &mut gs.ecs),

            // Grab item
            (VirtualKeyCode::G, ..) => get_item(&mut gs.ecs),

            // Show Inventory
            (VirtualKeyCode::I, ..) => return RunState::ShowInventory,

            // Drop item in inventory
            (VirtualKeyCode::D, ..) => return RunState::ShowDropItem,

            // Save Game
            (VirtualKeyCode::Escape, ..) => return RunState::SaveGame,

            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
        None => gamelog
            .entries
            .push("There is nothing here to pickup".to_string()),
    }
}
