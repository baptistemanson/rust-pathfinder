use crate::{character::Character, world::World};

mod find_target;
mod spell;
mod weapon;

use std::fmt;
// Value AI:
// 0 cannot be cast or pointless
// 10 gives an advantage
// 20 gives a major advantage
pub trait Activity: fmt::Debug {
    fn can_be_used(&self, character: &Character, context: &World) -> bool;
    fn ai_playing_value(&self, character: &Character, context: &World) -> i64;
    fn resolve(&self, character: &Character, context: &mut World);
    fn get_name(&self) -> &str;
}

/**
Right now doesnt care about the specificity of a character.
 */
impl<'a> Character<'a> {
    fn get_activities(&self) -> Vec<Box<dyn Activity>> {
        vec![
            Box::new(spell::ActivityAttackWithSpell::new()),
            Box::new(weapon::ActivityAttackWithWeapon::new()),
        ]
    }
}

/**
Return a new
*/
pub fn select_best_activity<'a>(character: &'a Character, world: &'a World) -> Box<dyn Activity> {
    let activities = character.get_activities();
    let best = activities
        .into_iter()
        .map(|act| (act.ai_playing_value(character, world), act))
        .max_by(|a, b| a.0.cmp(&b.0));
    match best {
        Some((_, c)) => c,
        None => panic!("Oh no"),
    }
}
