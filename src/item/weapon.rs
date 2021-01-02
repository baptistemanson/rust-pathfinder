use crate::character::Character;
use crate::dice;
use crate::item::traits::implementation::attack_ability_modifier;
use crate::item::traits::implementation::deadly;
use crate::item::traits::implementation::striking;
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
    pub damage_type: DamageType,
    pub dice_faces: i64,
    pub nb_dice: i64,
    pub striking_level: i64, // runes
    pub potency_level: i64,  // runes
}

#[derive(Copy, Clone, Debug)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
}

pub struct DamageRollResults {
    pub value: i64,
    pub damage_type: DamageType,
    pub is_critical: bool,
    pub details: String,
}

impl WeaponItem {
    pub fn damage_roll(&self, source: &Character, is_critical: bool) -> DamageRollResults {
        let CombatProperties {
            dice_faces,
            nb_dice,
            ..
        } = self.damage;

        // striking bonus: modify number of dices
        let striking_bonus = striking(self);

        // ability mod
        let ability_modifier = attack_ability_modifier(self, source);

        // deadly bonus: add flat after critical
        let deadly_bonus = deadly(self, is_critical);

        let roll = dice::dxx(dice_faces, nb_dice + striking_bonus.value);

        let total =
            (roll + ability_modifier.value) * if is_critical { 2 } else { 1 } + deadly_bonus.value;

        DamageRollResults {
            value: total,
            damage_type: self.damage.damage_type, // because I turned it to a copy type... no prob bob
            is_critical,
            details: if is_critical {
                format!(
                    "Critical 2x ({}d{}[{}]{}){} = {} dmg",
                    nb_dice + striking_bonus.value,
                    dice_faces,
                    roll,
                    ability_modifier.details,
                    deadly_bonus.details,
                    total
                )
            } else {
                format!(
                    "{}d{}[{}]{} = {} dmg",
                    nb_dice + striking_bonus.value,
                    dice_faces,
                    roll,
                    ability_modifier.details,
                    total
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
            damage_type: DamageType::Slashing,
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
            damage_type: DamageType::Bludgeoning,
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
            damage_type: DamageType::Bludgeoning,
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
            damage_type: DamageType::Piercing,
            dice_faces: 8,
            striking_level: 0,
            potency_level: 0,
        },
    }
}

pub fn sling() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            bulk: 2,
            name: String::from("Sling"),
            traits: TraitSet::from(Trait::Propulsive),
        },
        is_two_hands: false,
        is_ranged: true,
        range: 100,
        damage: CombatProperties {
            nb_dice: 1,
            dice_faces: 6,
            damage_type: DamageType::Bludgeoning,
            striking_level: 0,
            potency_level: 0,
        },
    }
}
