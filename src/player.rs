use specs::prelude::*;
use std::cmp::{max, min};
use tcod::input::Key;
use tcod::input::KeyCode::*;

use super::{xy_idx, Player, Position, State, TileType};

pub const SCREEN_WIDTH: usize = 60;
pub const SCREEN_HEIGHT: usize = 40;

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(SCREEN_WIDTH as i32 - 1, max(0, pos.x + delta_x));
            pos.y = min(SCREEN_HEIGHT as i32 - 1, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State) -> bool {
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
        Key { code: Escape, .. } => return true, // Exit the game

        Key { code: Left, .. } => try_move_player(-1, 0, &mut gs.ecs),
        Key { code: Right, .. } => try_move_player(1, 0, &mut gs.ecs),
        Key { code: Up, .. } => try_move_player(0, -1, &mut gs.ecs),
        Key { code: Down, .. } => try_move_player(0, 1, &mut gs.ecs),

        _ => {}
    }

    false
}
