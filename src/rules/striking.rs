use crate::{character::Character, roll::Roll, rules::RuleImplementation, world::World};

pub struct StrikingRule {
    pub level: usize,
}

impl RuleImplementation for StrikingRule {
    fn dmg_pre_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        let extra_die = if r.dices.len() > 0 {
            r.dices[0]
        } else {
            panic!("Striking cannot find the prev die")
        };
        r + Roll::new(self.level, extra_die, 0)
    }
}
