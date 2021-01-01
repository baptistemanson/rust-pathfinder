use crate::{character::Character, dice, world::World};

use super::{find_target::find_first_target, Activity};

#[derive(Clone, Debug)]
pub struct ActivityAttackWithWeapon<'a> {
    name: &'a str,
}

impl<'a> ActivityAttackWithWeapon<'a> {
    pub fn new() -> Self {
        Self { name: "Attack" }
    }
}

impl<'a> Activity for ActivityAttackWithWeapon<'a> {
    fn can_be_used(&self, character: &Character, _context: &World) -> bool {
        character.hp > 0
    }
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        dice::d20()
    }

    fn resolve<'lworld>(&self, character: &Character, world: &mut World) {
        let dmg = dice::d20();
        let target_id = find_first_target(&character.party, world);
        match target_id {
            None => {
                return;
            }
            Some(id) => {
                let target: &mut Character = world
                    .characters
                    .get_mut(&id)
                    .expect("Oh no, could not find the right target");
                println!(
                    "\t{} attacks {} for {} dmg",
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
