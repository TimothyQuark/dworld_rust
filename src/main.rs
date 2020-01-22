use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        geometry::Plane,
        math::{Point3, Vector2, Vector3},
        Named, Parent, Transform, TransformBundle,
    },
    ecs::{
        Component, Entities, Entity, Join, LazyUpdate, NullStorage, Read, ReadExpect, ReadStorage,
        System, WriteStorage,
    },
    input::{is_close_requested, is_key_down, InputBundle, InputHandler, StringBindings},
    prelude::*,
    renderer::{
        camera::{ActiveCamera, Camera, Projection},
        debug_drawing::DebugLinesComponent,
        formats::texture::ImageFormat,
        palette::Srgba,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        transparent::Transparent,
        types::DefaultBackend,
        RenderDebugLines, RenderFlat2D, RenderToWindow, RenderingBundle, Texture,
    },
    tiles::{CoordinateEncoder, FlatEncoder, MapStorage, RenderTiles2D, Tile, TileMap},
    utils::application_root_dir,
    window::ScreenDimensions,
    winit,
};

mod startup_state;
use startup_state::StartUpState;
mod console;

mod game_resources;
pub mod rendering;
use rendering::render::{RenderConsoleToScreen, RenderTile};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default()); // Amethyst logger

    let app_root = application_root_dir()?;
    let resources_directory = app_root.join("resources");

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())? // Bundle for handling input
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()) // Plugin for handling 2D rendering
                .with_plugin(RenderTiles2D::<RenderTile, FlatEncoder>::default()), // Plugin for handling 2D tilemaps
        )?
        .with(
            RenderConsoleToScreen::default(),
            "RenderConsoleToScreen",
            &[],
        );

    let mut game = Application::build(resources_directory, StartUpState)?.build(game_data)?;

    game.run();

    Ok(())
}
