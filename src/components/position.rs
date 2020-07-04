use amethyst::ecs::prelude::{Component, VecStorage};

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
