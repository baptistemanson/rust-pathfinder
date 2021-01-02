use crate::dice;
use rand::prelude::*;

use super::{GameItem, ItemInfo};
#[derive(Clone, Debug)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub damage: CombatProperties,
    pub is_two_hands: bool,
    pub is_ranged: bool,
    range: i64,
}

impl GameItem for WeaponItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Clone, Debug)]
pub struct CombatProperties {
    pub dice_faces: i64,
    pub nb_dice: i64,
    pub striking_level: i64, // runes
    pub potency_level: i64,  // runes
}

pub struct DamageRollResults {
    pub value: i64,
    pub is_critical: bool,
    pub details: String,
}

impl CombatProperties {
    pub fn roll(&self, is_critical: bool) -> DamageRollResults {
        let CombatProperties {
            dice_faces,
            nb_dice,
            striking_level,
            ..
        } = self;

        let roll = dice::dx(*dice_faces);
        let value = (roll * nb_dice + striking_level) * if is_critical { 2 } else { 1 };
        DamageRollResults {
            value: value,
            is_critical,
            details: if is_critical {
                format!(
                    "Critical 2x ⬡ {}d{} + {} = {}",
                    nb_dice, dice_faces, striking_level, value
                )
            } else {
                format!(
                    "⬡ {}d{} + {} = {}",
                    nb_dice, dice_faces, striking_level, value
                )
            },
        }
    }
}

// p280 greatsword
pub fn greatsword() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            name: String::from("Greatsword +1"),
            bulk: 2,
            traits: 0,
        },
        is_two_hands: true,
        is_ranged: false,
        range: 0,
        damage: CombatProperties {
            nb_dice: 1,
            striking_level: 1,
            potency_level: 1,
            dice_faces: 12,
        },
    }
}

// p280 fist
pub fn fist() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            bulk: 0,
            name: String::from("Fist"),
            traits: 0,
        },
        is_two_hands: false,
        is_ranged: false,
        range: 0,
        damage: CombatProperties {
            nb_dice: 1,
            striking_level: 0,
            potency_level: 0,
            dice_faces: 4,
        },
    }
}

pub fn unarmed() -> WeaponItem {
    let names = vec!["Fist", "Head", "Knee", "Foot"];
    let pick = thread_rng().gen_range(0..names.len());
    WeaponItem {
        info: ItemInfo {
            bulk: 0,
            name: String::from(names[pick]),
            traits: 0,
        },
        is_two_hands: false,
        is_ranged: false,
        range: 0,
        damage: CombatProperties {
            nb_dice: 1,
            striking_level: 0,
            potency_level: 0,
            dice_faces: 4,
        },
    }
}
pub fn longbow() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            bulk: 2,
            name: String::from("Longbow"),
            traits: 0,
        },
        is_two_hands: true,
        is_ranged: true,
        range: 100,
        damage: CombatProperties {
            nb_dice: 1,
            dice_faces: 8,
            striking_level: 0,
            potency_level: 0,
        },
    }
}
