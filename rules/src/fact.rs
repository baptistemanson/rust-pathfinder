use crate::timeline::CharacterId;

// Facts are things that just happened in the world.
// They serve as the platform to communicate between the rule engine and the view
// Mostly for animation purposes, so they are less detailed than the info log for now.

#[derive(Debug)]
pub enum Fact {
    InfoFact(String),
    Move {
        character_id: CharacterId,
        x: u32,
        y: u32,
    },
    CastSpell {
        character_id: CharacterId,
        spell_id: u32,
    },
    Attack {
        character_id: CharacterId,
        target_id: CharacterId,
        attack_type: u32,
        dmg: u32,
    },
    Unconscious(CharacterId),
}
#[derive(Debug)]
pub struct Facts {
    facts: Vec<Fact>,
}

impl Facts {
    pub fn info(&mut self, info: &str) {
        self.facts.push(Fact::InfoFact(String::from(info)));
    }

    pub fn new() -> Self {
        Self { facts: vec![] }
    }
}
