use dice::Roll;

use crate::{character::Character, rules::RuleImplementation, world::World};

pub struct StrikingRule {
    pub level: usize,
}

impl RuleImplementation for StrikingRule {
    fn dmg_pre_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        let extra_die = if r.get_bonus("weapon").nb_dice > 0 {
            r.get_bonus("weapon").face
        } else {
            panic!("Striking cannot find the prev die")
        };
        r + Roll::d("striking", self.level as i64, extra_die as i64)
    }
}
