use dice::Roll;

use crate::{
    character::Character,
    status::{Duration, StatusEffect, StatusType},
    world::World,
};

use super::{find_target::find_all_friends, Activity};

#[derive(Clone, Debug)]
pub struct Action {}

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
        Roll::from("1d20").roll()
    }

    fn resolve<'lworld>(&mut self, character: &Character, world: &mut World) {
        let keys: Vec<String> = find_all_friends(&character.party, world);
        let mut target_names: Vec<String> = vec![];
        for key in &keys {
            let target = world.get_mut_character(key);
            target.add_status(StatusEffect {
                duration: Duration::Round(10),
                status_type: StatusType::Bless,
            });
            target_names.push(target.name.clone());
        }
        println!("\t{} blessed {}", character.name, target_names.join(", "));
    }

    fn get_name(&self) -> &str {
        "Bless"
    }
}
