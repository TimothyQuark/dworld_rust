use super::{Map, Monster, Name, Position, Viewshed};
use bracket_geometry::prelude::Point;
use specs::prelude::*;
use tcod::pathfinding::AStar;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;
        for (mut viewshed, _monster, name, mut pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            viewshed.dirty = true;
            if viewshed.visible_tiles.contains(&*player_pos) {
                let fov_map = map.fov_map_mutex.lock().unwrap().clone(); // This is probably very costly
                let mut path = AStar::new_from_map(fov_map, 1.41);
                let path_found = path.find((pos.x, pos.y), (player_pos.x, player_pos.y));

                // Note that path.len is calculated before the monster moves.
                //println!("{} tiles from player (before moving)", path.len());
                if path_found && path.len() > 0 {
                    println!(
                        "{} sees and chases you! {} tiles from you",
                        name.name,
                        path.len()
                    );
                    let (x, y) = path.walk_one_step(true).unwrap();
                    // New monster position
                    let new_idx = map.xy_idx(x, y);
                    // If entity already in space monster tries to move to, it bumps
                    if map.is_walkable[new_idx] {
                        // Check if monster next to player. If yes, don't move monster
                        let distance = bracket_geometry::prelude::DistanceAlg::Pythagoras
                            .distance2d(Point::new(pos.x, pos.y), *player_pos);
                        if distance < 1.5 {
                            println!("{} shouts insults in your face!", name.name);
                        } else {
                            let old_idx = map.xy_idx(pos.x, pos.y); // Old monster position
                            map.is_walkable[old_idx] = true; // Old position now open
                            pos.x = x; // Monster moves
                            pos.y = y;
                            map.is_walkable[new_idx] = false; // New position is blocked
                        }
                    } else {
                        println!("{} bumps into something while chasing player", name.name);
                    }
                }
            }
        }
    }
}
