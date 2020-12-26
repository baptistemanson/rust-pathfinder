use crate::{character::Character, encounter_state::EncounterState};
use std::fmt;
// Value AI:
// 0 cannot be cast or pointless
// 10 gives an advantage
// 20 gives a major advantage

pub trait Activity: fmt::Debug {
    fn can_be_used(&self, character: &Character, context: &EncounterState) -> bool;
    fn ai_playing_value(&self, character: &Character, context: &EncounterState) -> i64;
    fn resolve(&self, context: &mut EncounterState);
    fn get_name(&self) -> &str;
}

#[derive(Clone, Debug)]
pub struct ActivityAttackWithWeapon<'a> {
    name: &'a str,
    //   character: &'a Character<'b>,
}
impl<'a> ActivityAttackWithWeapon<'a> {
    pub fn new() -> Self {
        Self { name: "Attack" }
    }
}
impl<'a> Activity for ActivityAttackWithWeapon<'a> {
    fn can_be_used(&self, character: &Character, _context: &EncounterState) -> bool {
        return character.hp > 0;
    }
    fn ai_playing_value(&self, _character: &Character, _context: &EncounterState) -> i64 {
        return 10;
    }

    fn resolve(&self, _context: &mut EncounterState) {
        todo![];
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

#[derive(Clone, Debug)]
pub struct ActivityAttackWithSpell<'a> {
    name: &'a str,
    // character: &'a Character<'b>,
}

impl<'a> ActivityAttackWithSpell<'a> {
    pub fn new() -> Self {
        Self { name: "Spell" }
    }
}

impl<'a> Activity for ActivityAttackWithSpell<'a> {
    fn can_be_used(&self, character: &Character, _context: &EncounterState) -> bool {
        return character.hp > 0;
    }
    fn ai_playing_value(&self, _character: &Character, _context: &EncounterState) -> i64 {
        return 10;
    }

    fn resolve(&self, _context: &mut EncounterState) {
        todo![];
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
