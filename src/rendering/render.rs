use crate::console::Console;
use crate::game_resources::GameInfo;
use amethyst::{
    core::math::Point3,
    core::Named,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::palette::Srgba,
    tiles::{FlatEncoder, MapStorage, Tile, TileMap},
    winit,
};

#[derive(Clone, Debug)]
pub struct RenderTile {
    pub glyph: usize,
    pub color: Srgba,
}

impl Default for RenderTile {
    fn default() -> Self {
        RenderTile {
            glyph: 0, // 0 is filled tiled, 46 is ., 32 is transparent tile
            color: Srgba::new(0.5, 0.5, 0.5, 1.0),
        }
    }
}

impl Tile for RenderTile {
    fn sprite(&self, _coord: Point3<u32>, _world: &World) -> Option<usize> {
        Some(self.glyph) // Default tile, used when first loaded
    }

    fn tint(&self, _pt: Point3<u32>, _world: &World) -> Srgba {
        self.color
    }
}

// Checks if the console tiles have changed, and if yes,
// redraws all render tiles on the screen
pub struct RenderConsoleToScreen {}

impl Default for RenderConsoleToScreen {
    fn default() -> Self {
        Self {}
    }
}

impl<'s> System<'s> for RenderConsoleToScreen {
    type SystemData = (
        WriteStorage<'s, TileMap<RenderTile, FlatEncoder>>,
        ReadStorage<'s, Named>,
        ReadExpect<'s, GameInfo>,
        WriteExpect<'s, Console>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut tilemaps, names, gameinfo, mut console, input): Self::SystemData) {
        
        // if input.key_is_down(winit::VirtualKeyCode::A) {
        //     let fg = Srgba::new(0.0, 1.0, 0.0, 1.0);
        //     let bg = Srgba::new(0.0, 0.0, 1.0, 1.0);
        //     console.print(1, 1, 12, fg, bg);
        //     console.print(2, 2, 1, fg, bg);
        //     console.print(3, 3, 3, fg, bg);
        //     console.print(4, 4, 5, fg, bg);
        //     console.print(5, 5, 7, fg, bg);
        //     console.print(53, 35, 255, fg, bg);
        // }

        // Console has not been modified, no need to redraw, exits system
        if !console.is_dirty {
            return;
        }

        let map_height = gameinfo.tilemap_height;
        let map_width = gameinfo.tilemap_width;

        for (map, name) in (&mut tilemaps, &names).join() {
            if name.name == "fg_map" {
                amethyst::tiles::iters::Region::new(
                    Point3::new(0, 0, 0),
                    Point3::new(map_width - 1, map_height - 1, 1),
                )
                .iter()
                .enumerate()
                .for_each(|(idx, coord)| {
                    if let Some(fg) = map.get_mut(&coord) {
                        fg.glyph = console.tiles[idx].glyph;
                        fg.color.color = console.tiles[idx].fg.color;
                    }
                })
            } else if name.name == "bg_map" {
                amethyst::tiles::iters::Region::new(
                    Point3::new(0, 0, 0),
                    Point3::new(map_width - 1, map_height - 1, 1),
                )
                .iter()
                .enumerate()
                .for_each(|(idx, coord)| {
                    if let Some(bg) = map.get_mut(&coord) {
                        bg.color.color = console.tiles[idx].bg.color;
                    }
                })
            } else {
                panic!(
                    "A tile map exists which is not being drawn to the screen: {}",
                    name.name
                );
            }
        }

        console.is_dirty = false;
    }
}
