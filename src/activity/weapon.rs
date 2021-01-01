use crate::item::DamageFormula;
use crate::item::DamageRollResults;
use crate::{character::Character, dice, world::World};

use super::{find_target::find_first_conscious_enemy, Activity};
use crate::slots::*;

#[derive(Clone, Debug)]
pub struct Action<'a> {
    name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Action { name: "Attack" }
    }
}

// p280 fist
fn fist() -> DamageFormula {
    DamageFormula::ClassicDamageFormula{nb_dice: 1, bonus:0, dice_faces: 4}
}
// @todo eventually, there will be some deduction and addition here
fn compute_damage_weapon(source: &Character, _target: &Character) -> DamageRollResults {
    match &source.slots.hands {
        HandSlot::SingleHands(Some(w), _) => w.damage.roll(),
        HandSlot::SingleHands(None, Some(w)) => w.damage.roll(),
        HandSlot::SingleHands(None, None) => fist().roll(),
        HandSlot::TwoHands(w) => w.damage.roll(),
        }
}
impl<'a> Activity for Action<'a> {
    
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        dice::d20()
    }

    fn resolve<'lworld>(&self, source: &Character, world: &mut World) {
        
        let target_id = find_first_conscious_enemy(&source.party, world);
        match target_id {
            None => {
                return;
            }
            Some(id) => {
                let target: &mut Character = world.get_mut_character(&id);
                let dmg_result = compute_damage_weapon(source, target);
                println!(
                    "\t{} attacks {} for {} damage ({})",
                    source.name, target.name, dmg_result.value, dmg_result.details
                );
                (*target).sub_hp(dmg_result.value);
            }
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
