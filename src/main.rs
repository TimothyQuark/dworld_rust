use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 60;
const SCREEN_HEIGHT: i32 = 40;

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

struct State {
    ecs: World,
    tcod: Tcod,
}
#[derive(Debug, Component)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(SCREEN_WIDTH - 1, max(0, pos.x + delta_x));
        pos.y = min(SCREEN_HEIGHT - 1, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State) {
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

        //Key { code: Escape, .. } => return true, // Exit the game

        Key { code: Left, .. } => try_move_player(-1, 0, &mut gs.ecs),
        Key { code: Right, .. } => try_move_player(1, 0, &mut gs.ecs),
        Key { code: Up, .. } => try_move_player(0, -1, &mut gs.ecs),
        Key { code: Down, .. } => try_move_player(0, 1, &mut gs.ecs),

        _ => {}
    }
}

impl State {
    fn tick(&mut self) {
        self.tcod.root.clear();

        player_input(self);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            self.tcod
                .root
                .put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
        }
        self.tcod.root.flush();
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
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("DWorld")
        .init();

    //let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod_temp = Tcod { root };
    tcod_temp.root.set_default_foreground(WHITE);

    let mut gs = State {
        ecs: World::new(),
        tcod: tcod_temp,
    };

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

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 5, y: 20 })
            .with(Renderable {
                glyph: 'g',
                fg: RED,
                bg: BLACK,
            })
            .build();
    }

    while !gs.tcod.root.window_closed() {
        gs.tick();
    }
}
