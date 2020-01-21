#[derive(Clone, Debug)]
pub struct GameInfo {
    pub tile_width: u32,
    pub tile_height: u32,
    pub tilemap_height: u32,
    pub tilemap_width: u32,
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            tile_width: 0,
            tile_height: 0,
            tilemap_height: 0,
            tilemap_width: 0,
        }
    }
}
