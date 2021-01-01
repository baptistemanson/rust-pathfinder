use crate::{character::Character, dice, world::World};

use super::{Activity};

#[derive(Clone, Debug)]
pub struct Action<'a> {
    name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Self { name: "Pass" }
    }
}

impl<'a> Activity for Action<'a> {
    fn can_be_used(&self, _character: &Character, _context: &World) -> bool {
        true
    }
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        1
    }

    fn resolve<'lworld>(&self, _character: &Character, _world: &mut World) {
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
