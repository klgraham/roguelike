use crate::components::{CanMelee, CombatStats, Name, SuffersDamage};
use rltk::console;
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CanMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SuffersDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ents, mut can_melee, names, combat_stats, mut suffer_damage) = data;

        for (_ent, can_melee, name, stats) in (&ents, &can_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                // do if this entity is not dead
                let target_stats = combat_stats.get(can_melee.target).unwrap();
                if target_stats.hp > 0 {
                    // target not dead
                    let target_name = names.get(can_melee.target).unwrap();
                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        console::log(&format!("{} can't hurt {:?}.", &name.name, &target_name));
                    } else {
                        console::log(&format!(
                            "{} hit {:?}, for {} hp.",
                            &name.name, &target_name, &damage
                        ));
                        SuffersDamage::new_damage(&mut suffer_damage, can_melee.target, damage);
                    }
                }
            }
        }

        can_melee.clear();
    }
}
