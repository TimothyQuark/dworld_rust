use amethyst::{
    core::{
        TransformBundle,
    },
    input::{InputBundle,StringBindings},
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderFlat2D, RenderToWindow, RenderingBundle,
    },
    tiles::{ FlatEncoder,  RenderTiles2D},
    utils::application_root_dir,
};

mod states;
use states::{StartUpState};

mod console_util;

mod game_resources;

mod components;

mod systems;
use systems::render_system::{RenderConsoleToScreen, RenderTile};

mod utilities;

//mod keyboard_test_system;
//use keyboard_test_system::KeyboardTestSystem;

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
    //.with(KeyboardTestSystem::default(), "KeyBoardTestSystem", &[]);

    let mut game =
        Application::build(resources_directory, StartUpState)?.build(game_data)?;
    game.run();
    Ok(())
}
