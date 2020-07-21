use super::{BlockTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlockTile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;
        // All changes to the map are corrected with this system. Hence no longer dirty
        map.dirty = false;
        // Recompute map, without considering entities
        map.recompute_map();

        // Check if an entity blocks a certain position, adds it to is_walkable
        for (position, _blocks) in (&position, &blockers).join() {
            let idx = map.xy_idx(position.x, position.y);
            // Blocks the position in is_walkable, but not the fov map!
            // Done so that monsters will try to follow player, instead
            // of finding very long alternate path. Happened often in narrow
            // corridors
            map.is_walkable[idx] = false;
        }
    }
}
