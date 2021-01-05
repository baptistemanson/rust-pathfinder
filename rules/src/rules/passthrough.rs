use dice::Roll;

use crate::{character::Character, rules::RuleImplementation, world::World};

pub struct Passthrough {}
impl RuleImplementation for Passthrough {
    fn dmg_pre_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        r
    }
}
