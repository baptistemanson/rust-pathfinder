use dice::Roll;

use crate::{
    character::Character,
    fact,
    status::{Duration, StatusEffect, StatusType},
    ui::log,
    world::World,
};

use super::{find_target::find_all_friends, Activity};

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
        Roll::d("", 1, 20).roll() / 4
    }

    fn resolve<'lworld>(
        &mut self,
        character: &Character,
        world: &mut World,
        facts: &mut fact::Facts,
    ) {
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
        facts.info(&format!(
            "\t{} blessed {}",
            character.name,
            target_names.join(", ")
        ));
    }

    fn get_name(&self) -> &str {
        "Bless"
    }
}
