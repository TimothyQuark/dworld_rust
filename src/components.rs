use bracket_geometry::prelude::Point;
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

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Debug, Component)]
pub struct Monster {}

#[derive(Debug, Component)]
pub struct Name {
    pub name: String,
}

#[derive(Debug, Component)]
pub struct BlockTile {}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub curr_hp: i32,
    pub armor: i32,
    pub magic_res: i32,
    pub max_mana: i32,
    pub curr_mana: i32,
}
