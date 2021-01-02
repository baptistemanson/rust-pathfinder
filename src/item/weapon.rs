use crate::dice;
use rand::prelude::*;

use super::{
    traits::{none, Trait, TraitSet},
    GameItem, ItemInfo,
};
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

impl WeaponItem {
    pub fn roll(&self, is_critical: bool) -> DamageRollResults {
        let CombatProperties {
            dice_faces,
            nb_dice,
            striking_level,
            ..
        } = self.damage;

        let roll = dice::dx(dice_faces);

        // deadly bonus
        let (deadly_bonus, deadly_details) =
            if is_critical && self.info.traits.contains(Trait::DeadlyD10) {
                // p282
                let nb_to_roll = match self.damage.striking_level {
                    0 => 1,
                    1 => 1,
                    2 => 2,
                    _ => 3,
                };
                let size_to_roll = dice_faces; //@todo put real formula
                let deadly_value = nb_to_roll * dice::dx(size_to_roll);
                (
                    deadly_value,
                    format!(" + {}d{} {} deadly", nb_to_roll, size_to_roll, deadly_value),
                )
            } else {
                (0, String::from(""))
            };

        let value =
            (roll * nb_dice + striking_level) * if is_critical { 2 } else { 1 } + deadly_bonus;

        DamageRollResults {
            value: value,
            is_critical,
            details: if is_critical {
                format!(
                    "Critical 2x ({}d{}[{}] + {}){} = {}",
                    nb_dice, dice_faces, roll, striking_level, deadly_details, value
                )
            } else {
                format!(
                    "{}d{}[{}] + {} = {}",
                    nb_dice, dice_faces, roll, striking_level, value
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
            traits: TraitSet::from(Trait::DeadlyD10),
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
            traits: none(),
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
            traits: none(),
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
            traits: none(),
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
