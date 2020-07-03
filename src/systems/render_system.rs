use amethyst::{
    core::math::Point3,
    core::Named,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    //input::{InputHandler, StringBindings},
    prelude::*,
    renderer::palette::Srgba,
    tiles::{FlatEncoder, MapStorage, Tile, TileMap},
};

use crate::console_util::console::Console;
use crate::game_resources::GameInfo;

#[derive(Clone, Debug)]
pub struct RenderTile {
    pub glyph: usize,
    pub color: Srgba,
}

impl Default for RenderTile {
    fn default() -> Self {
        RenderTile {
            glyph: 0,                              // 0 is filled tiled, 46 is ., 32 is transparent tile
            color: Srgba::new(1.0, 0.0, 0.0, 1.0), // Default red to show errors
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
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, TileMap<RenderTile, FlatEncoder>>,
        ReadStorage<'s, Named>,
        ReadExpect<'s, GameInfo>,
        WriteExpect<'s, Console>,
        //Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut tilemaps, names, gameinfo, mut console): Self::SystemData) {
        // Console has not been modified, no need to redraw, exits system
        if !console.is_dirty {
            return;
        }

        console.is_dirty = false;

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
    }
}
