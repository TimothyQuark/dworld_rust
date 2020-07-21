use bracket_geometry::prelude::Point;
use rand::Rng;
use specs::prelude::*;
use tcod::colors::*;
use tcod::console::*;

mod components;
pub use components::*;

mod map;
pub use map::*;

mod player;
pub use player::*;

mod rect;
pub use rect::Rect;

mod visibility_system;
pub use visibility_system::VisibilitySystem;

mod monster_ai_system;
pub use monster_ai_system::MonsterAI;

mod map_indexing_system;
pub use map_indexing_system::MapIndexingSystem;

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
    pub runstate: RunState,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
    ExitGame,
}

impl State {
    fn tick(&mut self) -> bool {
        self.tcod.root.clear(); // Clear the screen every tick

        let mut exit: bool = false;
        match self.runstate {
            RunState::Running => {
                self.run_systems();
                self.runstate = RunState::Paused;
            }
            RunState::Paused => {
                self.runstate = player_input(self);
                if self.runstate == RunState::ExitGame {
                    exit = true
                };
            }
            RunState::ExitGame => {
                exit = true;
            }
        }

        draw_map(&self.ecs, &mut self.tcod);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                self.tcod
                    .root
                    .put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
            }
        }
        self.tcod.root.flush();

        return exit; // if command given to quit game, returns true
    }

    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);

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
        runstate: RunState::Running,
    };

    let map: Map = Map::new_map_rooms_and_corridors();

    let (player_x, player_y) = map.rooms[0].center();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlockTile>();
    gs.ecs.register::<CombatStats>();

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
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true, // Force initial recompute
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats {
            max_hp: 30,
            curr_hp: 30,
            armor: 2,
            magic_res: 4,
            max_mana: 50,
            curr_mana: 50,
        })
        .build();

    let mut rng = rand::thread_rng();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: char;
        let name: String;
        let roll = rng.gen_range(1, 3); // Rolls 1 or 2
        match roll {
            1 => {
                glyph = 'g';
                name = "Goblin".to_string()
            }
            _ => {
                glyph = 'o';
                name = "Orc".to_string()
            }
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: glyph,
                fg: RED,
                bg: BLACK,
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(CombatStats {
                max_hp: 5,
                curr_hp: 5,
                armor: 2,
                magic_res: 4,
                max_mana: 50,
                curr_mana: 50,
            })
            .with(BlockTile {})
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));

    while !gs.tcod.root.window_closed() {
        let exit = gs.tick();
        if exit {
            break;
        }
    }
}
