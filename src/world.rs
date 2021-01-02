use std::collections::HashMap;

use crate::{character::Character, item::weapons::longbow, timeline::CharacterId};

/**
 * Cannot delete characters during an encounter...
 */
pub struct World<'world> {
    pub characters: HashMap<CharacterId, Character<'world>>,
}

impl<'world> World<'world> {
    pub fn new() -> Self {
        let kobold1 = Character::new("Kobold 1", "enemy", 40);
        let kobold2 = Character::new("Kobold 2", "enemy", 40);
        let mut kobold3 = Character::new("Kobold 3", "enemy", 40);
        kobold3.loadout.right_hand = Some(longbow());
        let paladin1 = Character::new("Paladin 1", "good guys", 300);
        let paladin2 = Character::new("Paladin 2", "good guys", 300);
        let mut characters = HashMap::new();
        characters.insert(String::from(kobold1.id), kobold1);
        characters.insert(String::from(kobold2.id), kobold2);
        characters.insert(String::from(kobold3.id), kobold3);
        characters.insert(String::from(paladin1.id), paladin1);
        characters.insert(String::from(paladin2.id), paladin2);
        World { characters }
    }
    pub fn get_characters(&self) -> Vec<&Character<'world>> {
        self.characters.values().collect()
    }
    pub fn get_character(&self, key: &str) -> &Character<'world> {
        self.characters
            .get(key)
            .expect("couldnt find the character to activate in the world!")
    }
    pub fn get_mut_character(&mut self, key: &str) -> &mut Character<'world> {
        self.characters
            .get_mut(key)
            .expect("Oh no, could not find the right target")
    }
}
