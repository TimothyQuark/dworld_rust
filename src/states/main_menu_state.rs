use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::Vector3, Named, Transform},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Texture,
    },
    tiles::{FlatEncoder, TileMap},
    window::ScreenDimensions,
    winit,
};

use crate::console_util::console::Console;
use crate::game_resources::GameInfo;
pub struct MainMenuState;

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Currently inside the MainMenuState!");

        let gameinfo = data.world.fetch_mut::<GameInfo>();
        let mut console = data.world.fetch_mut::<Console>();

        let title = String::from("DWorld: Dungeon World");
        let t_half_length: u32 = (title.len() / 2) as u32;

        // Print game title to center of screen, 20% from top of screen
        console.print_str(
            gameinfo.tilemap_width / 2 - t_half_length,
            gameinfo.tile_height / 5 as u32,
            &title,
        );
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let mut console = data.world.fetch_mut::<Console>();
        console.cls(); // Clear the screen initially
        
        Trans::None
    }
}
