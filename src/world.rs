use std::collections::HashMap;

use crate::{character::Character, timeline::CharacterId};

pub struct World<'a> {
    pub characters: HashMap<CharacterId, Character<'a>>,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        let kobold1 = Character::new("Kobold 1", "enemy", 40);
        let kobold2 = Character::new("Kobold 2", "enemy", 40);
        let kobold3 = Character::new("Kobold 3", "enemy", 40);
        let paladin = Character::new("Paladin", "good guys", 40);
        let mut characters = HashMap::new();
        characters.insert(String::from(kobold1.id), kobold1);
        characters.insert(String::from(kobold2.id), kobold2);
        characters.insert(String::from(kobold3.id), kobold3);
        characters.insert(String::from(paladin.id), paladin);
        World { characters }
    }
    pub fn get_characters(&'a self) -> Vec<&Character<'a>> {
        self.characters.values().collect()
    }
}
