use amethyst::{
    core::math::Point3,
    ecs::{Join, Read, System, WriteStorage},
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

impl<'s> System<'s> for UpdateConsoleSprites {
    type SystemData = (
        WriteStorage<'s, TileMap<ConsoleTile, FlatEncoder>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut tilemaps, input): Self::SystemData) {
        if input.key_is_down(winit::VirtualKeyCode::A) {
            let mut a = 1;
            for tilemap in (&mut tilemaps).join() {
                let point = Point3::new(30, 30, 0);
                //println!("A tilemap was found! {}", a);
                a += 1;
                let to_change = tilemap.get_mut(&point);
                if let Some(m) = &to_change {
                    to_change.unwrap().glyph += 1;
                } else {
                    println!("Point does not have corresponding tile");
                }
            }
        }
    }
}
