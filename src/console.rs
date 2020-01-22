use amethyst::renderer::palette::Srgba;

pub struct ConsoleTile {
    pub glyph: usize,
    pub fg: Srgba,
    pub bg: Srgba,
}

pub struct Console {
    pub width: u32,
    pub height: u32,
    total_tiles: u32,

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
            total_tiles: width * height,
            tiles,
            is_dirty: true, // Force initial redrawing of all tiles
        }
    }

    pub fn print(&mut self, x: u32, y: u32, glyph: usize, fg: Srgba, bg: Srgba) {
        // no need to check for x>= 0 since u32
        if x < self.width && y < self.height {
            self.is_dirty = true;

            let idx = (y * self.width + x) as usize;
            self.tiles[idx].glyph = glyph;
            self.tiles[idx].fg = fg;
            self.tiles[idx].bg = bg;
        }
    }
}
