use super::console::Console;
use crate::utilities::to_cp437;
use amethyst::{
    ecs::{Read, System, WriteExpect},
    input::{InputHandler, StringBindings},
    renderer::palette::Srgba,
    winit::VirtualKeyCode,
};

pub struct KeyboardTestSystem {
    idx: usize,
}

impl Default for KeyboardTestSystem {
    fn default() -> Self {
        Self { idx: 0 }
    }
}

// A system for testing out keyboard functions, and also for testing the console
// functions
impl<'s> System<'s> for KeyboardTestSystem {
    type SystemData = (
        WriteExpect<'s, Console>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut console, input): Self::SystemData) {
        for keys in input.keys_that_are_down() {
            match keys {
                VirtualKeyCode::A => {
                    let fg = Srgba::new(0.0, 1.0, 0.0, 1.0);
                    let bg = Srgba::new(0.0, 0.0, 1.0, 1.0);
                    console.set(1, 5, 66, fg, bg);

                    console.print(1, 1, 'a');
                    console.print(2, 2, 'b');
                    console.print(3, 3, '@');

                    console.print_cl(4, 4, 'Q', fg, bg);
                    console.print_cl(5, 5, ']', fg, bg);
                    console.print_cl(6, 6, '+', fg, bg);
                    console.print_cl(53, 35, '%', fg, bg);

                    console.print_str(10, 10, "Hello World");

                    console.print_str_cl(20, 20, "Hello World", fg, bg);
                }
                VirtualKeyCode::Q => {
                    console.cls();
                    self.idx = 0;
                }

                VirtualKeyCode::Z => {
                    let fg = Srgba::new(0.0, 0.5, 0.5, 1.0);
                    let bg = Srgba::new(0.5, 0.0, 0.0, 1.0);
                    let idx = self.idx as u32;
                    console.print(idx, 15, 'a');
                    console.print_cl(idx, 30, '&', fg, bg);
                    self.idx += 1;
                }

                VirtualKeyCode::D => {
                    let fg = Srgba::new(0.0, 0.5, 0.5, 1.0); // Dirty green blue
                    let bg = Srgba::new(0.5, 0.0, 0.0, 1.0); // Red

                    console.draw_fillbox(5, 5, 5, 5, to_cp437('*') as usize, fg, bg);

                    console.draw_box(30, 30, 5, 5, fg, bg);
                }

                _ => {}
            }
        }
    }
}
