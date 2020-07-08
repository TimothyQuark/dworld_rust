use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

mod components;
pub use components::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod rect;
pub use rect::Rect;

pub const SCREEN_WIDTH: usize = 60;
pub const SCREEN_HEIGHT: usize = 40;

const LIMIT_FPS: i32 = 144;

pub struct Tcod {
    root: Root,
    //con: Offscreen,
}

pub struct State {
    ecs: World,
    tcod: Tcod,
}

impl State {
    fn tick(&mut self) -> bool {
        self.tcod.root.clear(); // Clear the screen every tick

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

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // Create player
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
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
