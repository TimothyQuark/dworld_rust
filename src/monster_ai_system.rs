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
            if viewshed.visible_tiles.contains(&*player_pos) {
                println!("{} shouts insults!", name.name);

                let fov_map = map.fov_map_mutex.lock().unwrap().clone(); // This is probably very costly
                let mut path = AStar::new_from_map(fov_map, 1.41);
                let path_found = path.find((pos.x, pos.y), (player_pos.x, player_pos.y));

                if path_found && path.len() > 1 {
                    // If entity already in space monster tries to move to, it bumps                    
                    let (x, y) = path.walk_one_step(true).unwrap();
                    let new_idx = map.xy_idx(x, y); // New monster position
                    if map.is_walkable[new_idx] {
                        let old_idx = map.xy_idx(pos.x, pos.y); // Old monster position
                        map.is_walkable[old_idx] = true; // Old position now walkable
                        pos.x = x; // Monster moves
                        pos.y = y;
                        map.is_walkable[new_idx] = false; // New position is blocked
                    } else {
                        println!("Monster bumps into something while chasing player");
                    }
                    viewshed.dirty = true;
                }
            }
        }
    }
}
