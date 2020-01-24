use super::utilities::string_to_cp437;
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

    // Print a tile with a black bg and white fg
    pub fn print(&mut self, x: u32, y: u32, glyph: usize) {
        // no need to check for x>= 0 since u32
        self.is_dirty = true;
        if x < self.width && y < self.height {
            self.is_dirty = true;

            let idx = (y * self.width + x) as usize;
            self.tiles[idx].glyph = glyph;
            self.tiles[idx].fg = Srgba::new(1.0, 1.0, 1.0, 1.0);
            self.tiles[idx].bg = Srgba::new(0.0, 0.0, 0.0, 1.0);
        }
    }

    // Print a tile with a bg and fg color
    pub fn print_cl(&mut self, x: u32, y: u32, glyph: usize, fg: Srgba, bg: Srgba) {
        // no need to check for x>= 0 since u32
        self.is_dirty = true;
        if x < self.width && y < self.height {
            self.is_dirty = true;

            let idx = (y * self.width + x) as usize;
            self.tiles[idx].glyph = glyph;
            self.tiles[idx].fg = fg;
            self.tiles[idx].bg = bg;
        }
    }

    // Print a string with white fg and black bg, assuming it
    // fits on one row of the console
    pub fn print_str(&mut self, x: u32, y: u32, output: &str) {
        self.is_dirty = true;

        let bytes = string_to_cp437(output);
        let mut idx = (y * self.width + x) as usize;

        for glyph in bytes {
            // Check if out of console bounds
            if idx < self.tiles.len() {
                // bytes is u8 because cp437 only supports 0-255
                self.tiles[idx].glyph = glyph as usize;
                self.tiles[idx].fg = Srgba::new(1.0, 1.0, 1.0, 1.0);
                self.tiles[idx].bg = Srgba::new(0.0, 0.0, 0.0, 1.0);

                idx += 1;
            }
        }
    }

    // Print a string with chosen fg bg, assuming it
    // fits on one row of the console
    pub fn print_str_cl(&mut self, x: u32, y: u32, output: &str, fg: Srgba, bg: Srgba) {
        self.is_dirty = true;

        let bytes = string_to_cp437(output);
        let idx = (y * self.width + x) as usize;

        for glyph in bytes {
            // Check if out of console bounds
            if idx < self.tiles.len() {
                // bytes is u8 because cp437 only supports 0-255
                self.tiles[idx].glyph = glyph as usize;
                self.tiles[idx].fg = fg;
                self.tiles[idx].bg = bg;
            }
        }
    }

    // Clean the screen
    pub fn cls(&mut self) {
        self.is_dirty = true;
        let fg_color = Srgba::new(1.0, 1.0, 1.0, 1.0);
        let bg_color = Srgba::new(0.0, 0.0, 0.0, 1.0);

        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = fg_color;
            tile.bg = bg_color;
        }
    }
}
