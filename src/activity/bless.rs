use crate::{
    character::{Character, Duration, StatusEffect, StatusType},
    dice,
    world::World,
};

use super::{find_target::find_all_friends, Activity};

#[derive(Clone, Debug)]
pub struct Action<'a> {
    name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Self { name: "Bless" }
    }
}

impl<'a> Activity for Action<'a> {
    fn can_be_used(&self, _character: &Character, _context: &World) -> bool {
        true
    }
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        dice::dx(10)
    }

    fn resolve<'lworld>(&self, character: &Character, world: &mut World) {
        let keys: Vec<String> = find_all_friends(character.party, world);
        for key in &keys {
            let target = world.get_mut_character(key);
            target.add_status(StatusEffect {
                duration: Duration::Round(2),
                status_type: StatusType::Bless,
            });
            println!("\t{} blessed {}", character.name, target.name)
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
