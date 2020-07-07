use rand::Rng;
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: usize = 60;
const SCREEN_HEIGHT: usize = 40;

const LIMIT_FPS: i32 = 144;

struct Tcod {
    root: Root,
    //con: Offscreen,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: char,
    fg: Color,
    bg: Color,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct State {
    ecs: World,
    tcod: Tcod,
}
#[derive(Debug, Component)]
struct Player {}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREEN_WIDTH as usize) + x as usize
}

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

fn player_input(gs: &mut State) -> bool {
    let tcod = &mut gs.tcod;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {
            code: Enter,
            alt: true,
            .. // Ignore all other fields of struct
        } => {
            // Toggle to fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        Key { code: Escape, .. } => return true, // Exit the game

        Key { code: Left, .. } => try_move_player(-1, 0, &mut gs.ecs),
        Key { code: Right, .. } => try_move_player(1, 0, &mut gs.ecs),
        Key { code: Up, .. } => try_move_player(0, -1, &mut gs.ecs),
        Key { code: Down, .. } => try_move_player(0, 1, &mut gs.ecs),

        _ => {}
    }

    false
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; SCREEN_WIDTH * SCREEN_HEIGHT];

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

fn draw_map(map: &[TileType], tcod: &mut Tcod) {
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

impl State {
    fn tick(&mut self) -> bool {
        self.tcod.root.clear();

        let exit = player_input(self);

        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, &mut self.tcod);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            self.tcod
                .root
                .put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
        }
        self.tcod.root.flush();

        return exit; // if command given to quit game, returns true
    }

    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("cp437_20x20.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("DWorld")
        .init();

    //let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod_temp = Tcod { root };
    tcod_temp.root.set_default_foreground(WHITE);

    let mut gs = State {
        ecs: World::new(),
        tcod: tcod_temp,
    };

    gs.ecs.insert(new_map());

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // Create player
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: '@',
            fg: WHITE,
            bg: BLACK,
        })
        .with(Player {})
        .build();

    while !gs.tcod.root.window_closed() {
        let exit = gs.tick();
        if exit {
            break;
        }
    }
}
