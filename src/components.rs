use specs::prelude::*;
use specs_derive::*;
use tcod::colors::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Debug, Component)]
pub struct Player {}
