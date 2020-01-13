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
    tiles::{MapStorage, MortonEncoder2D, RenderTiles2D, Tile, TileMap, CoordinateEncoder},
    utils::application_root_dir,
    window::ScreenDimensions,
    winit,
};

#[derive(Default)]
struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
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

fn init_screen_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(-250.0, -245.0, 0.1);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .named("screen_reference")
        .build()
}

fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .named("reference")
        .build()
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}

fn init_camera(world: &mut World, parent: Entity, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(Parent { entity: parent })
        .with(camera)
        .named("camera")
        .build()
}

#[derive(Clone)]
struct ExampleTile {
    glyph_num : usize,
}

impl Default for ExampleTile {

    fn default() -> Self {
        Self {glyph_num : 0}
    }
}

impl Tile for ExampleTile {
    fn sprite(&self, _coord: Point3<u32>, _world: &World) -> Option<usize> {
        Some(self.glyph_num) // Default tile, used when first loaded
    }

}



struct ChangeMapTiles {}

impl Default for ChangeMapTiles {
    fn default() -> Self {
        Self {}
    }
}

impl<'s> System<'s> for ChangeMapTiles {
    type SystemData = (
        WriteStorage<'s, TileMap<ExampleTile, MortonEncoder2D> >,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut tilemaps, input): Self::SystemData) {
        if input.key_is_down(winit::VirtualKeyCode::A) {
            let mut a = 1;
            for tilemap in (&mut tilemaps).join() {
                let point = Point3::new(20, 20, 0);
                //println!("A tilemap was found! {}", a);
                a += 1;
                let to_change = tilemap.get_mut(&point);
                if let Some(m) = &to_change {
                    to_change.unwrap().glyph_num += 1;
                }
                else {
                    println!("Point does not have corresponding tile");
                }
                
            }
        }
    }
}

struct InitialState;

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Player>();
        world.register::<Named>();
        world.register::<TileMap<ExampleTile>>();

        let font_sprite_handle = load_sprite_sheet(world, "cp437_20x20.png", "cp437_20x20.ron");
        // let tiles_sprite_handle = load_sprite_sheet(
        //     world,
        //     "../resources/cp437_20x20.png",
        //     "../resources/cp437_20x20.ron",
        // ); // Keep same for now

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _reference = init_reference_sprite(world, &font_sprite_handle);
        let player = init_player(world, &font_sprite_handle);

        let _camera = init_camera(
            world,
            player,
            Transform::from(Vector3::new(0.0, 0.0, 1.1)),
            Camera::standard_2d(width, height),
        );

        let map = TileMap::<ExampleTile, MortonEncoder2D>::new(
            Vector3::new(48, 48, 1),  // Dimensions
            Vector3::new(20, 20, 1),  // Tile dimensions
            Some(font_sprite_handle), // Sprite sheet
        );

        let map_entity = world
            .create_entity()
            .with(map)
            .with(Transform::default())
            .build();
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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

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
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder2D>::default()), // Plugin for handling 2D tilemaps
        )?
        .with(ChangeMapTiles::default(), "ChangeMapTiles", &[]);

    let mut game = Application::build(resources_directory, InitialState)?.build(game_data)?;
    game.run();

    Ok(())
}
