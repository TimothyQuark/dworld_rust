use super::{Map, Monster, Name, Position, Viewshed};
use crate::{RunState, WantsToMelee};
use bracket_geometry::prelude::Point;
use specs::prelude::*;
use tcod::pathfinding::AStar;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, WantsToMelee>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            runstate,
            entities,
            mut viewshed,
            monster,
            mut position,
            name,
            mut wants_to_melee,
        ) = data;

        if *runstate != RunState::MonsterTurn {
            return;
        } // End system if not a monster turn

        for (entity, mut viewshed, _monster, name, mut pos) in
            (&entities, &mut viewshed, &monster, &name, &mut position).join()
        {
            viewshed.dirty = true;
            let distance = bracket_geometry::prelude::DistanceAlg::Pythagoras
                .distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                println!("Monster attacks you!");
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack");
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                let fov_map = map.fov_map_mutex.lock().unwrap().clone(); // This is probably very costly
                let mut path = AStar::new_from_map(fov_map, 1.41);
                let path_found = path.find((pos.x, pos.y), (player_pos.x, player_pos.y));

                if path_found && path.len() > 0 {
                    println!(
                        "{} sees and chases you! {} tiles from you (before moving)",
                        name.name,
                        path.len()
                    );
                    let (x, y) = path.walk_one_step(true).unwrap();
                    // New monster position
                    let new_idx = map.xy_idx(x, y);
                    // If entity already in space monster tries to move to, it bumps
                    if map.is_walkable[new_idx] {
                        let old_idx = map.xy_idx(pos.x, pos.y); // Old monster position
                        map.is_walkable[old_idx] = true; // Old position now open
                        pos.x = x; // Monster moves
                        pos.y = y;
                        map.is_walkable[new_idx] = false; // New position is blocked
                    } else {
                        println!("{} bumps into something while chasing player", name.name);
                    }
                }
            }
        }
    }
}
