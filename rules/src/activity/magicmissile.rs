use dice::Roll;

use crate::{character::Character, world::World};

use super::{find_target::find_first_conscious_enemy, Activity};

#[derive(Clone, Debug)]
pub struct Action;
impl Action {
    pub fn new() -> Self {
        Self {}
    }
}

impl Activity for Action {
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        10
    }

    fn resolve(&mut self, character: &Character, world: &mut World) {
        let dmg = Roll::d("", 1, 20).roll();
        let target_id = find_first_conscious_enemy(&character.party, world);
        match target_id {
            None => {
                return;
            }
            Some(id) => {
                let target: &mut Character = world.get_mut_character(&id);
                println!(
                    "\t{} casts a magic missile to {} for {} dmg",
                    character.name, target.name, dmg
                );
                (*target).sub_hp(dmg);
            }
        }
    }

    fn get_name(&self) -> &str {
        "Magic Missile"
    }

    fn get_cost(&self) -> i64 {
        3
    }
}
