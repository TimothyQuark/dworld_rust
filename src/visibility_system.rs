use super::{Map, Player, Position, Viewshed};
use bracket_geometry::prelude::Point;
use specs::prelude::*;
use tcod::map::*;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();

                {
                    // Recompute the fov map for the entity in question.
                    let mut fov = map.fov_map_mutex.lock().unwrap();
                    fov.compute_fov(pos.x, pos.y, viewshed.range, true, FOV_ALGO);

                    for y in 0..map.height {
                        for x in 0..map.width {
                            let visible = fov.is_in_fov(x, y);
                            if visible {
                                viewshed.visible_tiles.push(Point::new(x, y));
                            }
                        }
                    }

                    viewshed
                        .visible_tiles
                        .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
                }
                // If this is the player, reveal what the can see
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
