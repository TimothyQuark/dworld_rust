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
    con: Offscreen,
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
    tcod: Tcod
}

impl State {
    fn tick(&mut self) {
        self.tcod.root.clear();
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            self.tcod.root.put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
        }
        self.tcod.root.flush();

        //handle_keys(self.tcod, player_x, player_y)
        let key = self.tcod.root.wait_for_keypress(true);

    }
}

fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
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

        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,

        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("cp437_20x20.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("DWorld")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod_temp = Tcod { root, con };
    tcod_temp.root.set_default_foreground(WHITE);

    let mut gs = State { ecs: World::new(), tcod: tcod_temp  };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: '@',
            fg: WHITE,
            bg: BLACK,
        })
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
