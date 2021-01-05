use crate::{character::Character, roll::Roll, rules::RuleImplementation, world::World};

pub struct Passthrough {}
impl RuleImplementation for Passthrough {
    fn dmg_pre_crit(&self, r: Roll, c: &Character, _: &World) -> Roll {
        r
    }
}
