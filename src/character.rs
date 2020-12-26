use crate::abilities::{get_default_abilities, AbilityScore};

#[derive(Clone, Debug)]
pub struct Character<'a> {
    pub max_hp: i64,
    pub hp: i64,
    pub name: &'a str,
    pub initiative: i64,
    pub ability_score: AbilityScore,
}

impl<'a> Character<'a> {
    pub fn create(name: &str, max_hp: i64) -> Character {
        Character {
            initiative: 0,
            max_hp,
            hp: max_hp,
            ability_score: get_default_abilities(),
            name, // I think it is copied here, because name is a slice
        }
    }

    // pub fn add_hp(&mut self, hp: i64) {
    //     self.hp = self.hp.saturating_add(hp);
    //     if self.hp > self.max_hp {
    //         self.hp = self.max_hp;
    //     }
    // }

    // we can get negative HP in pathfinder
    pub fn sub_hp(&mut self, hp: i64) {
        self.hp = self.hp.saturating_sub(hp);
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
}
