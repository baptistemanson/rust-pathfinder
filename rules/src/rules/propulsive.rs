use dice::Roll;

use crate::{
    character::Character, rules::RuleImplementation, timeline::get_modifier, world::World,
};

pub struct PropulsiveRule {}
impl RuleImplementation for PropulsiveRule {
    fn dmg_pre_crit(&self, r: Roll, c: &Character, _: &World) -> Roll {
        let str_mod = get_modifier(c.ability_score.strength);
        if str_mod >= 0 {
            //i64 divide rounds down,as per rule p444 Chapter 9
            r + Roll::flat("propulsive", str_mod / 2)
        } else {
            r + Roll::flat("propulsive", str_mod)
        }
    }
}
#[test]
fn cool() {
    assert_eq!(6 / 5, 1);
    assert_eq!(7 / 5, 1);
    assert_eq!(8 / 5, 1);
    assert_eq!(9 / 5, 1);
    assert_eq!(10 / 5, 2);
}
