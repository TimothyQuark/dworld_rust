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

use super::RenderTile;
use crate::console::Console;
use crate::game_resources::GameInfo;

// The initial game state, called when the program opens up. Does things such
// as create the consoles tiles.
pub struct StartUpState;

impl SimpleState for StartUpState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Named>();
        world.register::<TileMap<RenderTile>>();

        world.insert(GameInfo::default());

        // Keep these the same
        let fg_sprite_handle =
            load_sprite_sheet(world, "cp437_20x20_transparent.png", "cp437_20x20.ron");
        let bg_front_handle =
            load_sprite_sheet(world, "cp437_20x20_transparent.png", "cp437_20x20.ron");

        // Dimensions of the game window
        let (screen_width, screen_height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        init_camera(world, screen_width, screen_height); // Init the camera from window dimensions

        let font_size = (20.0, 20.0); // TODO: Have this read from a RON file, then store in game_info

        if screen_width % font_size.0 != 0.0 || screen_height % font_size.1 != 0.0 {
            panic!(
                "
            Screen width and screen height must be divisible by tile dimensions.
            Screen Width: {}
            Screen Height: {}
            Tile Width: {}
            Tile Height: {}
            ",
                screen_width, screen_height, font_size.0, font_size.1
            );
        } else {
            let map_width_tiles = (screen_width / font_size.0) as u32;
            let map_height_tiles = (screen_height / font_size.1) as u32;

            // Save this info as a resource to make it accessible for entire game
            // In different scope to keep borrower happy
            {
                let mut game_info = world.write_resource::<GameInfo>();
                game_info.tile_width = font_size.0 as u32;
                game_info.tile_height = font_size.1 as u32;
                game_info.tilemap_width = map_width_tiles as u32;
                game_info.tilemap_height = map_height_tiles as u32;
            }

            // The order in which these entities are created matters: cannot find a different
            // way to set rendering order. Later maps have priority, i.e draw over other maps
            let bg_map = TileMap::<RenderTile, FlatEncoder>::new(
                Vector3::new(map_width_tiles, map_height_tiles, 1), // Dimensions (# of tiles)
                Vector3::new(font_size.0 as u32, font_size.1 as u32, 1), // Tile dimensions
                Some(bg_front_handle),                              // Sprite sheet
            );

            let mut bg_map_transform = Transform::default();
            bg_map_transform.set_translation_xyz(
                (screen_width * 0.5) + (font_size.0 * 0.5),
                (screen_height * 0.5) - (font_size.1 * 0.5),
                1.0,
            );
            let _bg_map_entity = world
                .create_entity()
                .with(bg_map)
                .with(bg_map_transform)
                .named("bg_map")
                .build();

            let fg_map = TileMap::<RenderTile, FlatEncoder>::new(
                Vector3::new(map_width_tiles, map_height_tiles, 1), // Dimensions (# of tiles and z-levels (usually just 1 level))
                Vector3::new(font_size.0 as u32, font_size.1 as u32, 1), // Tile dimensions
                Some(fg_sprite_handle),                             // Sprite sheet
            );

            let mut fg_map_transform = Transform::default();
            fg_map_transform.set_translation_xyz(
                (screen_width * 0.5) + (font_size.0 * 0.5),
                (screen_height * 0.5) - (font_size.1 * 0.5),
                1.0,
            );
            let _fg_map_entity = world
                .create_entity()
                .with(fg_map)
                .with(fg_map_transform)
                .named("fg_map")
                .build();

            let console = Console::init(map_width_tiles, map_height_tiles);

            // Console is a global resource
            world.insert(console);
        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_camera(world: &mut World, screen_width: f32, screen_height: f32) {
    let mut transform = Transform::default();
    let width = screen_width; // in pixels
    let height = screen_height; // in pixels
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            0.0,
            5.0,
        )))
        .with(transform)
        .build();
}
