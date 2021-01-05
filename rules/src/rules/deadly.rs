use crate::{character::Character, roll::Roll, rules::RuleImplementation, world::World};

pub struct DeadlyRule {
    pub die: usize,
}

impl RuleImplementation for DeadlyRule {
    fn dmg_post_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        // @todo should only occur when it is a crit and boost roll based on striking level.
        // let nb_to_roll = match weapon.damage.striking_level {
        //     0 => 1,
        //     1 => 1,
        //     2 => 2,
        //     _ => 3,
        // };
        r + Roll::new(1, self.die as i64, 0)
    }
}
