use crate::rules::Rule;
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
    pub damage_type: DamageType,
    pub dice_faces: i64,
    pub nb_dice: usize,
    pub striking_level: i64, // runes
    pub potency_level: i64,  // runes
}

#[derive(Copy, Clone, Debug)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
}

// p280 greatsword
pub fn greatswordplus2() -> WeaponItem {
    WeaponItem {
        info: ItemInfo::new("Greatsword +2", 2, vec![Rule::Striking(2)]),
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

pub fn greatswordplus1() -> WeaponItem {
    WeaponItem {
        info: ItemInfo::new("Greatsword +2", 2, vec![Rule::Striking(1)]),
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
#[allow(dead_code)]
pub fn fist() -> WeaponItem {
    WeaponItem {
        info: ItemInfo::new("Fist", 0, vec![]),
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
        info: ItemInfo::new(names[pick], 0, vec![]),
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
        info: ItemInfo::new("Longbow", 2, vec![]),
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
        info: ItemInfo::new("Sling", 2, vec![Rule::Propulsive]),
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
