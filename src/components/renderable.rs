use amethyst::ecs::prelude::{Component, VecStorage};
use amethyst::renderer::palette::Srgba;

pub struct Renderable {
    glyph: char,
    fg: Srgba,
    bg: Srgba,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}
