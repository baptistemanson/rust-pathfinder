use crate::{character::Character, fact, world::World};

mod attack;
mod bless;
mod find_target;
mod magicmissile;
mod pass;

use std::fmt;
// Value AI:
// 0 cannot be cast or pointless
// 10 gives an advantage
// 20 gives a major advantage
pub trait Activity: fmt::Debug {
    fn can_be_used(&self, character: &Character, _context: &World) -> bool {
        character.hp > 0
    }
    fn ai_playing_value(&self, character: &Character, context: &World) -> i64;
    fn resolve(&mut self, character: &Character, context: &mut World, facts: &mut fact::Facts);
    fn get_name(&self) -> &str;
    fn get_cost(&self) -> i64 {
        1
    }
}

/**
Right now doesnt care about the specificity of a character.
Would be based on a list of feats and the equipment.
 */
impl Character {
    fn get_activities(&self) -> Vec<Box<dyn Activity>> {
        vec![
            Box::new(magicmissile::Action::new()),
            Box::new(attack::Action::new()),
            Box::new(bless::Action::new()),
        ]
    }
}

/**
Return a new
*/
pub fn select_best_action<'a>(
    character: &'a Character,
    action_left: i64,
    world: &'a World,
) -> Box<dyn Activity> {
    let activities = character.get_activities();
    let best = activities
        .into_iter()
        .filter(|act| act.can_be_used(character, world))
        .filter(|act| act.get_cost() <= action_left)
        .map(|act| (act.ai_playing_value(character, world), act))
        .max_by(|a, b| a.0.cmp(&b.0));
    match best {
        Some((_, c)) => c,
        None => Box::new(pass::Action::new()),
    }
}
