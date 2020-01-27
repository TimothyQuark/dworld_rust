use super::utilities::{string_to_cp437, to_cp437};
use amethyst::renderer::palette::Srgba;

pub struct ConsoleTile {
    pub glyph: usize,
    pub fg: Srgba,
    pub bg: Srgba,
}

pub struct Console {
    pub width: u32,
    pub height: u32,

    pub tiles: Vec<ConsoleTile>,
    pub is_dirty: bool,
}

impl Console {
    pub fn init(width: u32, height: u32) -> Console {
        let num_tiles: usize = (width * height) as usize;

        let mut tiles: Vec<ConsoleTile> = Vec::with_capacity(num_tiles);

        for _ in 0..num_tiles {
            tiles.push(ConsoleTile {
                glyph: 32, // Fully transparent tile. Fg uses this tile, bg is colored 0 tile
                fg: Srgba::new(1.0, 1.0, 1.0, 1.0), // White and fully opaque
                bg: Srgba::new(0.0, 0.0, 0.0, 1.0), // Black and fully opaque
            })
        }

        Console {
            width,
            height,
            tiles,
            is_dirty: true, // Force initial redrawing of all tiles
        }
    }

    // Print a tile using a cp437 code, also all extra tiles not in cp437
    pub fn set(&mut self, x: u32, y: u32, glyph: usize, fg: Srgba, bg: Srgba) {
        self.is_dirty = true;

        // no need to check for x>= 0 since u32
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.tiles[idx].glyph = glyph;
            self.tiles[idx].fg = fg;
            self.tiles[idx].bg = bg;
        }
    }

    /// Print a tile with a text symbol. Use set if using a tile number
    pub fn print(&mut self, x: u32, y: u32, glyph: char) {
        let output = to_cp437(glyph) as usize;
        let fg = Srgba::new(1.0, 1.0, 1.0, 1.0);
        let bg = Srgba::new(0.0, 0.0, 0.0, 1.0);

        self.set(x, y, output, fg, bg);
    }

    /// Print a tile with a bg and fg color
    pub fn print_cl(&mut self, x: u32, y: u32, glyph: char, fg: Srgba, bg: Srgba) {
        let output = to_cp437(glyph) as usize;

        self.set(x, y, output, fg, bg);
    }

    /// Print a string with white fg and black bg, assuming it
    /// fits on one row of the console
    pub fn print_str(&mut self, x: u32, y: u32, output: &str) {
        let bytes = string_to_cp437(output);
        let mut idx = (y * self.width + x) as usize;
        let fg = Srgba::new(1.0, 1.0, 1.0, 1.0); // White
        let bg = Srgba::new(0.0, 0.0, 0.0, 1.0); // Black

        for glyph in bytes {
            // Check if out of console bounds
            if idx < self.tiles.len() {
                self.set(idx as u32, y, glyph as usize, fg, bg);
                idx += 1;
            }
        }
    }

    /// Print a string with chosen fg bg, assuming it
    /// fits on one row of the console
    pub fn print_str_cl(&mut self, x: u32, y: u32, output: &str, fg: Srgba, bg: Srgba) {
        let bytes = string_to_cp437(output);
        let mut idx = (y * self.width + x) as usize;

        for glyph in bytes {
            // Check if out of console bounds
            if idx < self.tiles.len() {
                self.set(idx as u32, y, glyph as usize, fg, bg);

                idx += 1;
            }
        }
    }

    /// Clear the screen
    pub fn cls(&mut self) {
        self.is_dirty = true;
        let fg = Srgba::new(1.0, 1.0, 1.0, 1.0);
        let bg = Srgba::new(0.0, 0.0, 0.0, 1.0);

        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = fg;
            tile.bg = bg;
        }
    }

    /// Draw a box without fill using a custom tile
    #[allow(clippy::too_many_arguments)]
    pub fn draw_fillbox(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        glyph: usize,
        fg: Srgba,
        bg: Srgba,
    ) {
        for y_idx in y..y + height {
            for x_idx in x..x + width {
                self.set(x_idx, y_idx, glyph, fg, bg);
            }
        }
    }

    pub fn draw_box(&mut self, x: u32, y: u32, width: u32, height: u32, fg: Srgba, bg: Srgba) {
        self.print_cl(x, y, '┌', fg, bg);
        self.print_cl(x + width, y, '┐', fg, bg);
        self.print_cl(x, y + height, '└', fg, bg);
        self.print_cl(x + width, y + height, '┘', fg, bg);

        for x_idx in x + 1..x + width {
            self.print_cl(x_idx, y, '-', fg, bg);
            self.print_cl(x_idx, y + height, '-', fg, bg);
        }
        for y_idx in y + 1..y + height {
            self.print_cl(x, y_idx, '|', fg, bg);
            self.print_cl(x + width, y_idx, '|', fg, bg);
        }
    }
}
