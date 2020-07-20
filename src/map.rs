use rand::Rng;
use specs::prelude::*;
use std::cmp::{max, min};
use std::sync::Mutex;
use tcod::colors::*;
use tcod::console::*;
use tcod::map::Map as FovMap;

use super::{Rect, Tcod};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub fov_map_mutex: Mutex<FovMap>, // The fov map for ALL entities. To use, recompute fov for the entity in question
    pub revealed_tiles: Vec<bool>,    // Revealed tiles for PLAYER. Used to render map to console
    pub visible_tiles: Vec<bool>, // Currently visible tiles for PLAYER. Non visible but explored are greyed out
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    // If the map is changed, this function must be called
    fn recompute_fov(&mut self) {
        let mut fov_map = self.fov_map_mutex.lock().unwrap();

        for x in 0..60 {
            for y in 0..40 {
                let idx = self.xy_idx(x, y);
                let tile = &self.tiles[idx];
                match tile {
                    TileType::Wall => fov_map.set(x, y, false, false),
                    TileType::Floor => fov_map.set(x, y, true, true),
                }
            }
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 60 * 40],
            rooms: Vec::new(),
            width: 60,
            height: 40,
            fov_map_mutex: Mutex::new(FovMap::new(60, 40)),
            revealed_tiles: vec![false; 60 * 40],
            visible_tiles: vec![false; 60 * 40],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = rand::thread_rng();

        for _i in 0..MAX_ROOMS {
            let w = rng.gen_range(MIN_SIZE, MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE, MAX_SIZE);

            let x: i32 = rng.gen_range(1, 60 - w - 1) - 1;
            let y: i32 = rng.gen_range(1, 40 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.gen_range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }
        {
            // Recompute the fov map for the first time
            map.recompute_fov();
        }

        map
    }
}

pub fn draw_map(ecs: &World, tcod: &mut Tcod) {
    //let mut viewsheds = ecs.write_storage::<Viewshed>();
    //let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;

    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending on the tile type
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = '.';
                    fg = DARK_AMBER;
                }
                TileType::Wall => {
                    glyph = '#';
                    fg = DARKER_GREEN;
                }
            }

            if !map.visible_tiles[idx] {
                fg = LIGHT_GREY
            }
            tcod.root.put_char_ex(x, y, glyph, fg, BLACK);
        }

        x += 1;
        if x > 59 {
            x = 0;
            y += 1;
        }
    }
}
