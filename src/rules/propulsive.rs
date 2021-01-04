use crate::{
    character::Character, roll::Roll, rules::RuleImplementation, timeline::get_modifier,
    world::World,
};

pub struct PropulsiveRule {}
impl RuleImplementation for PropulsiveRule {
    fn attack_ability_modifier(&self, r: Roll, c: &Character, w: &World) -> Roll {
        let str_mod = get_modifier(c.ability_score.strength);
        //@todo check rounding rules for propulsive
        if str_mod >= 0 {
            r + (str_mod / 2)
        } else {
            r + str_mod
        }
    }
}
