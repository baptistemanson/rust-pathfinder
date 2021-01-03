use crate::{
    character::Character,
    item::{armor, weapon::sling, AnyItem, GameItem, ItemId},
};
use std::collections::HashMap;

use crate::timeline::CharacterId;

/**
 * Cannot delete characters during an encounter...
 */
pub struct World<'world> {
    pub characters: HashMap<CharacterId, Character<'world>>,
    pub items: HashMap<ItemId, AnyItem>,
}

impl<'world> World<'world> {
    pub fn new() -> Self {
        let mut items: HashMap<ItemId, AnyItem> = HashMap::new();
        let mut characters = HashMap::new();

        let mut enemy = Character::new("Kobold Monk", "enemy", 40);
        let armor = armor::leather();
        enemy.loadout.armor = Some(armor.info.id.clone());
        items.insert(armor.info.id.clone(), AnyItem::ArmorItem(armor));
        characters.insert(String::from(enemy.id), enemy);

        let mut enemy = Character::new("Kobold Slinger", "enemy", 40);
        let armor = armor::leather();
        enemy.loadout.armor = Some(armor.info.id.clone());
        items.insert(armor.info.id.clone(), AnyItem::ArmorItem(armor));
        let weapon = sling();
        enemy.loadout.right_hand = Some(weapon.info.id.clone());
        items.insert(weapon.info.id.clone(), AnyItem::WeaponItem(weapon));
        characters.insert(String::from(enemy.id), enemy);

        let mut ranger = Character::new("Kobold Ranger", "enemy", 40);
        ranger.ability_score.dexterity = 14;
        characters.insert(String::from(ranger.id), ranger);

        let mut paladin = Character::new("Paladin Leader", "good guys", 300);
        paladin.ability_score.strength = 18;
        characters.insert(String::from(paladin.id), paladin);

        let mut cavalier = Character::new("Cavalier", "good guys", 300);
        cavalier.ability_score.strength = 14;
        characters.insert(String::from(cavalier.id), cavalier);

        World { characters, items }
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
    pub fn tick_down(&mut self) {
        self.characters.iter_mut().for_each(|(_s, c)| c.tick_down());
    }
}
