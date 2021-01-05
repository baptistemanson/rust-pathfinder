use crate::{
    character::Character, roll::Roll, rules::RuleImplementation, timeline::get_modifier,
    utils::get_active_weapon, world::World,
};

pub struct StrengthModDamageRule {}
impl RuleImplementation for StrengthModDamageRule {
    fn dmg_pre_crit(&self, r: Roll, c: &Character, w: &World) -> Roll {
        let weapon = get_active_weapon(c, w);
        if weapon.is_ranged {
            return r;
        }
        let str_mod = get_modifier(c.ability_score.strength);
        r + str_mod
    }
}
