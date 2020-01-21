use crate::game_resources::GameInfo;
use amethyst::{
    core::math::Point3,
    core::Named,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::palette::Srgba,
    tiles::{FlatEncoder, MapStorage, Tile, TileMap},
    winit,
};

#[derive(Clone)]
pub struct ConsoleTile {
    pub glyph: usize,
    pub color: Srgba,
}

impl Default for ConsoleTile {
    fn default() -> Self {
        ConsoleTile {
            glyph: 46,
            color: Srgba::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl Tile for ConsoleTile {
    fn sprite(&self, _coord: Point3<u32>, _world: &World) -> Option<usize> {
        Some(self.glyph) // Default tile, used when first loaded
    }

    fn tint(&self, _pt: Point3<u32>, _world: &World) -> Srgba {
        self.color
    }
}

pub struct UpdateConsoleSprites {}

impl Default for UpdateConsoleSprites {
    fn default() -> Self {
        Self {}
    }
}

// This is just a system for testing out things
impl<'s> System<'s> for UpdateConsoleSprites {
    type SystemData = (
        WriteStorage<'s, TileMap<ConsoleTile, FlatEncoder>>,
        ReadStorage<'s, Named>,
        ReadExpect<'s, GameInfo>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut tilemaps, names, gameinfo, input): Self::SystemData) {
        if input.key_is_down(winit::VirtualKeyCode::A) {
            let map_height = gameinfo.tilemap_height;
            let map_width = gameinfo.tilemap_width;

            for (map, name) in (&mut tilemaps, &names).join() {
                if name.name == "fg_map" {
                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 0),
                        Point3::new(map_width - 1, map_height - 1, 1),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(fg) = map.get_mut(&coord) {
                            fg.glyph += 1;
                            fg.color.color.red = 1.0;
                            fg.color.color.green = 0.0;
                            fg.color.color.blue = 0.0;
                        }
                    })
                }
                else if name.name == "bg_map" {
                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 0),
                        Point3::new(map_width - 1, map_height - 1, 1),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(fg) = map.get_mut(&coord) {
                            fg.glyph = 0;
                            fg.color.color.red = 0.0;
                            fg.color.color.green = 0.0;
                            fg.color.color.blue = 1.0;
                        }
                    })
                }
            }
        }
    }
}
