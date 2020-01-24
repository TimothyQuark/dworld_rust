use super::console::Console;
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
                    console.print(1, 1, 12);
                    console.print(2, 2, 1);
                    console.print(3, 3, 3);
                    console.print_cl(4, 4, 5, fg, bg);
                    console.print_cl(5, 5, 7, fg, bg);
                    console.print_cl(53, 35, 255, fg, bg);

                    console.print_str(10, 10, "Hello World");

                    console.print_str_cl(10, 10, "Hello World", fg, bg);
                },
                VirtualKeyCode::Q => {
                    console.cls();
                    self.idx = 0;
                },

                VirtualKeyCode::Z => {
                    let fg = Srgba::new(0.0, 0.5, 0.5, 1.0);
                    let bg = Srgba::new(0.5, 0.0, 0.0, 1.0);
                    let idx = self.idx as u32;
                    console.print(idx, 15, 64);
                    console.print_cl(idx, 30, idx as usize, fg, bg);
                    self.idx += 1;

                },

                _ => {}
            }
        }
    }
}
