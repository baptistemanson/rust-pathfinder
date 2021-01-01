use crate::{character::Character, dice, world::World};

use super::{find_target::find_first_target, Activity};

#[derive(Clone, Debug)]
pub struct ActivityAttackWithSpell<'a> {
    name: &'a str,
    // character: &'a Character<'b>,
}

impl<'a> ActivityAttackWithSpell<'a> {
    pub fn new() -> Self {
        Self { name: "Spell" }
    }
}

impl<'a> Activity for ActivityAttackWithSpell<'a> {
    fn can_be_used(&self, character: &Character, _context: &World) -> bool {
        character.hp > 0
    }
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        10
    }

    fn resolve(&self, character: &Character, world: &mut World) {
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
                println!("{} attacks {} for {} dmg", character.name, target.name, dmg);
                (*target).sub_hp(dmg);
            }
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
