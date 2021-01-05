use dice::Roll;

use crate::{character::Character, rules::RuleImplementation, world::World};

pub struct StrikingRule {
    pub level: usize,
}

impl RuleImplementation for StrikingRule {
    fn dmg_pre_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        let extra_die = if r.get_bonus("weapon").dices.len() > 0 {
            r.get_bonus("weapon").dices[0]
        } else {
            panic!("Striking cannot find the prev die")
        };
        r + Roll::d("striking", self.level, extra_die)
    }
}
