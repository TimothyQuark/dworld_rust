use super::{CombatStats, Name, SufferDamage, WantsToMelee, GameLog};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut wants_to_melee, names, combat_stats, mut inflict_damage) = data;

        for (_entity, wants_to_melee, name, stats) in
            (&entities, &wants_to_melee, &names, &combat_stats).join()
        {
            if stats.curr_hp > 0 {
                let target_stats = combat_stats.get(wants_to_melee.target).unwrap();
                if target_stats.curr_hp > 0 {
                    let target_name = names.get(wants_to_melee.target).unwrap();

                    let damage = i32::max(0, stats.power - target_stats.armor);

                    if damage == 0 {
                        log.entries.push(format!(" {} is unable to hurt {}", &name.name, &target_name.name));
                    } else {
                        log.entries.push(format!(
                            " {} hit {} for {} hp.",
                            &name.name, &target_name.name, damage
                        ));
                        SufferDamage::new_damage(
                            &mut inflict_damage,
                            wants_to_melee.target,
                            damage,
                        );
                    }
                }
            }
        }

        wants_to_melee.clear();
    }
}
