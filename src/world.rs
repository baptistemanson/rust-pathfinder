use crate::{
    character::Character,
    item::{
        armor::{leather, scale_mail, ArmorItem},
        weapon::{greatsword, longbow, sling, WeaponItem},
        AnyItem, ItemId,
    },
};
use std::collections::HashMap;

use crate::timeline::CharacterId;

type WeaponSpawner = &'static dyn Fn() -> WeaponItem;
type ArmorSpawner = &'static dyn Fn() -> ArmorItem;

/**
 * Cannot delete characters during an encounter...
 */
pub struct World {
    pub characters: HashMap<CharacterId, Character>,
    pub items: HashMap<ItemId, AnyItem>,
}

impl World {
    pub fn new() -> Self {
        let items: HashMap<ItemId, AnyItem> = HashMap::new();
        let characters = HashMap::new();
        World { characters, items }
    }

    pub fn spawn_armor(&mut self, f: ArmorSpawner) -> Option<ItemId> {
        let item = f();
        let id = item.info.id.clone();
        self.items.insert(id.clone(), AnyItem::ArmorItem(item));
        Some(id)
    }

    pub fn spawn_weapon(&mut self, f: WeaponSpawner) -> Option<ItemId> {
        let item = f();
        let id = item.info.id.clone();
        self.items.insert(id.clone(), AnyItem::WeaponItem(item));
        Some(id)
    }

    pub fn get_characters(&self) -> Vec<&Character> {
        self.characters.values().collect()
    }
    pub fn get_character(&self, key: &str) -> &Character {
        self.characters
            .get(key)
            .expect("couldnt find the character to activate in the world!")
    }
    pub fn get_mut_character(&mut self, key: &str) -> &mut Character {
        self.characters
            .get_mut(key)
            .expect("Oh no, could not find the right target")
    }
    pub fn tick_down(&mut self) {
        self.characters.iter_mut().for_each(|(_s, c)| c.tick_down());
    }
}

pub fn init(world: &mut World) {
    init_unit(world, "Kobold Slinger", "kobolds", 40, &sling, &leather);
    init_unit(world, "Kobold Archer", "kobolds", 40, &longbow, &leather);
    init_unit(world, "Paladin", "knights", 400, &greatsword, &scale_mail);
}

fn init_unit(
    world: &mut World,
    name: &str,
    party: &str,
    hp: i64,
    weapon: WeaponSpawner,
    armor: ArmorSpawner,
) -> CharacterId {
    let mut character = Character::new(String::from(name), String::from(party), hp);
    let id = world.spawn_armor(armor);
    character.loadout.armor = id;

    let id = world.spawn_weapon(weapon);
    character.loadout.right_hand = id;
    let char_id = character.id.clone();
    let out = char_id.clone();
    world.characters.insert(char_id, character);
    out
}
