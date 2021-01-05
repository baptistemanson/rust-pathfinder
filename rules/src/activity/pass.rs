use crate::{character::Character, world::World};

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

    fn resolve<'lworld>(&mut self, _character: &Character, _world: &mut World) {}

    fn get_name(&self) -> &str {
        "Pass"
    }
}
