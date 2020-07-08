use rand::Rng;
use std::cmp::{max, min};
use tcod::colors::*;
use tcod::console::*;

use super::{Rect, Tcod};

pub const SCREEN_WIDTH: usize = 60;
pub const SCREEN_HEIGHT: usize = 40;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH as usize) + x as usize
}

fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; SCREEN_WIDTH * SCREEN_HEIGHT];

    // Walls surround the perimeter of the dungeon
    for x in 0..SCREEN_WIDTH as i32 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, SCREEN_HEIGHT as i32 - 1)] = TileType::Wall;
    }

    for y in 0..SCREEN_HEIGHT as i32 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(SCREEN_WIDTH as i32 - 1, y)] = TileType::Wall;
    }

    let mut rng = rand::thread_rng();

    for _i in 0..400 {
        let x: i32 = rng.gen_range(1, SCREEN_WIDTH as i32);
        let y: i32 = rng.gen_range(1, SCREEN_HEIGHT as i32);

        let idx = xy_idx(x, y);
        if idx != xy_idx(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < SCREEN_HEIGHT * SCREEN_WIDTH {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < SCREEN_WIDTH * SCREEN_HEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; SCREEN_HEIGHT * SCREEN_WIDTH];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = rand::thread_rng();

    for _i in 0..MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE, MAX_SIZE);
        let h = rng.gen_range(MIN_SIZE, MAX_SIZE);

        let x: i32 = rng.gen_range(1, SCREEN_WIDTH as i32 - w - 1) - 1;
        let y: i32 = rng.gen_range(1, SCREEN_HEIGHT as i32 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }

        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.gen_range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

pub fn draw_map(map: &[TileType], tcod: &mut Tcod) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => tcod.root.put_char_ex(x, y, '.', LIGHT_GREY, BLACK),
            TileType::Wall => tcod.root.put_char_ex(x, y, '#', DARK_GREEN, BLACK),
        }

        x += 1;
        if x > SCREEN_WIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}
