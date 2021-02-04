use crate::{character::Character, fact, world::World};

use super::Activity;

#[derive(Clone, Debug)]
pub struct Action;

impl Action {
    pub fn new() -> Self {
        Self {}
    }
}

impl Activity for Action {
    fn can_be_used(&self, _character: &Character, _context: &World) -> bool {
        true
    }

    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        1
    }

    fn resolve<'lworld>(
        &mut self,
        _character: &Character,
        _world: &mut World,
        _facts: &mut fact::Facts,
    ) {
    }

    fn get_name(&self) -> &str {
        "Pass"
    }
}
