use crate::abilities::{get_default_abilities, AbilityScore};

#[derive(Debug, Clone)]
pub struct Character<'a> {
    pub id: &'a str,
    pub max_hp: i64,
    pub name: &'a str,
    pub party: &'a str,
    pub hp: i64,
    pub initiative: i64,
    pub ability_score: AbilityScore,
}

impl<'a> Character<'a> {
    pub fn new(name: &'a str, party: &'a str, max_hp: i64) -> Character<'a> {
        Character {
            id: name,
            name,
            party,
            initiative: 0,
            max_hp,
            hp: max_hp,
            ability_score: get_default_abilities(),
        }
    }

    // pub fn add_hp(&mut self, hp: i64) {
    //     self.hp = self.hp.saturating_add(hp);
    //     if self.hp > self.max_hp {
    //         self.hp = self.max_hp;
    //     }
    // }

    // we can get negative HP in pathfinder
    #[allow(dead_code)]
    pub fn sub_hp(&mut self, hp: i64) {
        self.hp = self.hp.saturating_sub(hp);
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
}
