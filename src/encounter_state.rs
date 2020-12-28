use crate::character::Character;

pub struct EncounterState {
    pub turn_number: i64,
}

impl EncounterState {
    pub fn is_encounter_done(&self, participants: &Vec<Character>) -> bool {
        for e in participants {
            if e.hp > 0 {
                return false;
            }
        }
        return true;
    }

    pub fn roll_initiative(&mut self, participants: &mut Vec<Character>) {
        for character in participants.iter_mut() {
            character.roll_initiative();
            println!(
                "{} rolled initiative {}",
                character.name, character.initiative
            )
        }
    }
}
