use crate::{
    character::Character, roll::Roll, rules::RuleImplementation, timeline::get_modifier,
    world::World,
};

pub struct FinessRule {}
impl RuleImplementation for FinessRule {
    fn attack_ability_modifier(&self, r: Roll, c: &Character, w: &World) -> Roll {
        let str_mod = get_modifier(c.ability_score.strength);
        let dex_mod = get_modifier(c.ability_score.dexterity);
        if str_mod < dex_mod {
            r + (dex_mod - str_mod) // replace str by dex
        } else {
            r
        }
    }
}
