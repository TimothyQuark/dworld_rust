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

use super::{ConsoleTile};
use crate::game_resources::{GameInfo};

// The initial game state, called when the program opens up. Does things such
// as create the consoles tiles.
pub struct StartUpState;

impl SimpleState for StartUpState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Named>();
        world.register::<TileMap<ConsoleTile>>();

        world.insert(GameInfo::default());

        let font_sprite_handle = load_sprite_sheet(world, "cp437_20x20.png", "cp437_20x20.ron");
        // let tiles_sprite_handle = load_sprite_sheet(
        //     world,
        //     "../resources/cp437_20x20.png",
        //     "../resources/cp437_20x20.ron",
        // ); // Keep same for now

        // Dimensions of the game window
        let (screen_width, screen_height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = init_camera(world, screen_width, screen_height); // Init the camera from window dimensions

        let mut map_transform = Transform::default();
        let font_size = (20.0, 20.0); // TODO: Have this read from a RON file, then store in game_info

        if screen_width % font_size.0 != 0.0 || screen_height % font_size.1 != 0.0 {
            panic!("
            Screen width and screen height must be divisible by tile dimensions.
            Screen Width: {}
            Screen Height: {}
            Tile Width: {}
            Tile Height: {}
            ", screen_width, screen_height, font_size.0, font_size.1);
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
            


            let map = TileMap::<ConsoleTile, FlatEncoder>::new(
                Vector3::new(map_width_tiles, map_height_tiles, 1), // Dimensions (# of tiles)
                Vector3::new(20, 20, 1),                            // Tile dimensions
                Some(font_sprite_handle),                           // Sprite sheet
            );

            map_transform.set_translation_xyz(
                (screen_width * 0.5) + (font_size.0 * 0.5),
                (screen_height * 0.5) - (font_size.1 * 0.5),
                0.0,
            );
            let _map_entity = world.create_entity().with(map).with(map_transform).build();
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
