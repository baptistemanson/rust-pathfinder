use crate::character::Character;

pub struct EncounterState<'a, 'b> {
    pub participants: Vec<&'a mut Character<'b>>,
    pub turn_number: i64,
}

impl<'a, 'b> EncounterState<'a, 'b> {
    pub fn is_encounter_done(&mut self) -> bool {
        for e in self.participants.iter_mut() {
            if e.hp > 0 {
                return false;
            }
        }
        return true;
    }

    pub fn roll_initiative(&mut self) {
        for character in self.participants.iter_mut() {
            character.roll_initiative();
            println!(
                "{} rolled initiative {}",
                character.name, character.initiative
            )
        }
        self.participants
            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }
}
