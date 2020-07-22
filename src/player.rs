use specs::prelude::*;
use std::cmp::{max, min};
use tcod::input::Key;
use tcod::input::KeyCode::*;

use super::RunState;
use super::{Map, Player, Position, State, Viewshed};
use crate::{CombatStats, WantsToMelee};
use bracket_geometry::prelude::Point;

pub const SCREEN_WIDTH: usize = 60;
pub const SCREEN_HEIGHT: usize = 40;

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entites = ecs.entities();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in
        (&entites, &mut players, &mut positions, &mut viewsheds).join()
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
                return; // Prevents entity from moving after attacking
            }
        }

        if map.is_walkable[destination_idx] {
            pos.x = min(SCREEN_WIDTH as i32 - 1, max(0, pos.x + delta_x));
            pos.y = min(SCREEN_HEIGHT as i32 - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true; // Player has moved, recompute fov

            // Update player position resource
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State) -> RunState {
    let tcod = &mut gs.tcod;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        // Don't use fullscreen mode because it messes up resolution in Linux
        // Key {
        //     code: Enter,
        //     alt: true,
        //     .. // Ignore all other fields of struct
        // } => {
        //     // Toggle to fullscreen, Alt + Enter
        //     let fullscreen = tcod.root.is_fullscreen();
        //     tcod.root.set_fullscreen(!fullscreen);
        // }
        Key { code: Escape, .. } => return RunState::ExitGame, // Exit the game

        // Diagonal commands. Evaluated first since using Shift-Left etc controls
        Key {
            code: Left,
            shift: true,
            ..
        }
        | Key { code: NumPad7, .. }
        | Key {
            code: Char,
            printable: 'q',
            ..
        } => try_move_player(-1, -1, &mut gs.ecs),

        Key {
            code: Right,
            shift: true,
            ..
        }
        | Key { code: NumPad9, .. }
        | Key {
            code: Char,
            printable: 'e',
            ..
        } => try_move_player(1, -1, &mut gs.ecs),

        Key {
            code: Left,
            ctrl: true,
            ..
        }
        | Key { code: NumPad1, .. }
        | Key {
            code: Char,
            printable: 'z',
            ..
        } => try_move_player(-1, 1, &mut gs.ecs),

        Key {
            code: Right,
            ctrl: true,
            ..
        }
        | Key { code: NumPad3, .. }
        | Key {
            code: Char,
            printable: 'c',
            ..
        } => try_move_player(1, 1, &mut gs.ecs),

        // Basic directions
        Key { code: Left, .. }
        | Key { code: NumPad4, .. }
        | Key {
            code: Char,
            printable: 'a',
            ..
        } => try_move_player(-1, 0, &mut gs.ecs),

        Key { code: Right, .. }
        | Key { code: NumPad6, .. }
        | Key {
            code: Char,
            printable: 'd',
            ..
        } => try_move_player(1, 0, &mut gs.ecs),

        Key { code: Up, .. }
        | Key { code: NumPad8, .. }
        | Key {
            code: Char,
            printable: 'w',
            ..
        } => try_move_player(0, -1, &mut gs.ecs),

        Key { code: Down, .. }
        | Key { code: NumPad2, .. }
        | Key {
            code: Char,
            printable: 's',
            ..
        } => try_move_player(0, 1, &mut gs.ecs),

        _ => return RunState::AwaitingInput,
    }

    RunState::PlayerTurn
}
