use crate::{character::Character, item::weapon::sling};
use std::collections::HashMap;

use crate::{item::weapon::longbow, timeline::CharacterId};

/**
 * Cannot delete characters during an encounter...
 */
pub struct World<'world> {
    pub characters: HashMap<CharacterId, Character<'world>>,
}

impl<'world> World<'world> {
    pub fn new() -> Self {
        let mut kobold_monk = Character::new("Kobold Monk", "enemy", 40);
        kobold_monk.loadout.right_hand = None;
        kobold_monk.loadout.left_hand = None;

        let mut kobold_sling = Character::new("Kobold Sling", "enemy", 40);
        kobold_sling.loadout.right_hand = Some(sling());

        let mut ranger = Character::new("Kobold Ranger", "enemy", 40);
        ranger.loadout.right_hand = Some(longbow());
        ranger.ability_score.dexterity = 14;

        let mut paladin = Character::new("Paladin Leader", "good guys", 300);
        paladin.ability_score.strength = 18;

        let mut cavalier = Character::new("Cavalier", "good guys", 300);
        cavalier.ability_score.strength = 14;

        let mut characters = HashMap::new();
        characters.insert(String::from(kobold_monk.id), kobold_monk);
        characters.insert(String::from(kobold_sling.id), kobold_sling);
        characters.insert(String::from(ranger.id), ranger);
        characters.insert(String::from(paladin.id), paladin);
        characters.insert(String::from(cavalier.id), cavalier);
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
