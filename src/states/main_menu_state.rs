use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::palette::Srgba,
    winit::VirtualKeyCode,
};

use crate::console_util::{selection_box, Console};
use crate::game_resources::GameInfo;
#[derive(Default)]
pub struct MainMenuState {
    pub curr_menu_sel: u32, // Currently selected option
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Currently inside the MainMenuState!");
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let gameinfo = data.world.fetch_mut::<GameInfo>();
        let mut console = data.world.fetch_mut::<Console>();

        console.cls(); // Clear the screen initially

        // Print game title to center of screen, printed 1/4 from screen top
        let mut text = "Dworld";
        let mut t_half_length: u32 = (text.len() / 2) as u32;
        console.print_str(
            gameinfo.tilemap_width / 2 - t_half_length,
            gameinfo.tile_height / 4 as u32,
            &text,
        );

        text = "Dungeon World";
        t_half_length = (text.len() / 2) as u32;
        console.print_str(
            gameinfo.tilemap_width / 2 - t_half_length,
            gameinfo.tile_height / 4 + 1 as u32,
            &text,
        );

        //Print edgy comment, just under title
        text = "The dungeon does not rest";
        t_half_length = (text.len() / 2) as u32;
        console.print_str_cl(
            gameinfo.tilemap_width / 2 - t_half_length,
            gameinfo.tile_height / 4 + 5 as u32,
            &text,
            Srgba::new(0.78, 0.68, 0.5, 1.0),
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        );

        // Draw the menu options
        let non_sel_col = Srgba::new(1.0, 1.0, 1.0, 1.0); // White
        let sel_col = Srgba::new(1.0, 0.0, 0.0, 1.0); // Red
        let menu_options: Vec<String> = vec![
            "Load Game".to_string(),
            "New Game".to_string(),
            "Settings".to_string(),
            "About".to_string(),
        ];
        selection_box(
            &mut console,
            gameinfo.tilemap_width / 2,
            gameinfo.tilemap_height / 2, //Print to middle of screen
            menu_options,
            self.curr_menu_sel,
            non_sel_col,
            sel_col,
        );

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    //Trans::Push(Box::new(PauseMenuState::default()))
                    println!("Escape key was pressed, exiting game");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Down) {
                    // If current selection is index 0, ignore key
                    if self.curr_menu_sel != 3 {
                        self.curr_menu_sel += 1;
                    }
                    Trans::None
                } else if is_key_down(&event, VirtualKeyCode::Up) {
                    // If current selection is index 3, ignore key
                    if self.curr_menu_sel != 0 {
                        self.curr_menu_sel -= 1;
                    }
                    Trans::None
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => Trans::None,
            StateEvent::Input(input) => Trans::None,
        }
    }
}
