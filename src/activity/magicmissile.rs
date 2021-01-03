use crate::{character::Character, dice, world::World};

use super::{find_target::find_first_conscious_enemy, Activity};

#[derive(Clone, Debug)]
pub struct Action<'a> {
    name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Self {
            name: "Magic Missile",
        }
    }
}

impl<'a> Activity for Action<'a> {
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        10
    }

    fn resolve(&self, character: &Character, world: &mut World) {
        let dmg = dice::d20();
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
        self.name
    }
}
