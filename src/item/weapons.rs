use crate::dice;

use super::{GameItem, ItemInfo};
#[derive(Clone, Debug)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub damage: CombatProperties,
    pub is_two_hands: bool,
    pub is_ranged: bool,
}

impl GameItem for WeaponItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Clone, Debug)]
pub enum CombatProperties {
    Standard {
        dice_faces: i64,
        nb_dice: i64,
        striking_level: i64,
        potency_level: i64,
    },
    Advanced {},
}

pub struct DamageRollResults {
    pub value: i64,
    pub is_critical: bool,
    pub details: String,
}

impl CombatProperties {
    pub fn roll(&self, is_critical: bool) -> DamageRollResults {
        match self {
            CombatProperties::Advanced {} => DamageRollResults {
                value: 0,
                is_critical: false,
                details: format!("Not implemented"),
            },
            CombatProperties::Standard {
                dice_faces,
                nb_dice,
                striking_level: bonus,
                ..
            } => {
                let roll = dice::dx(*dice_faces);
                let value = (roll * nb_dice + bonus) * if is_critical { 2 } else { 1 };
                DamageRollResults {
                    value: value,
                    is_critical,
                    details: if is_critical {
                        format!(
                            "Critical 2x ⚅ {}d{} + {} = {}",
                            nb_dice, dice_faces, bonus, value
                        )
                    } else {
                        format!("⚅ {}d{} + {} = {}", nb_dice, dice_faces, bonus, value)
                    },
                }
            }
        }
    }
}

// p280 greatsword
pub fn greatsword() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            name: String::from("Greatsword +1"),
            bulk: 2,
        },
        is_two_hands: true,
        is_ranged: false,
        damage: CombatProperties::Standard {
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
        },
        is_two_hands: false,
        is_ranged: false,
        damage: CombatProperties::Standard {
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
        },
        is_two_hands: true,
        is_ranged: false,
        damage: CombatProperties::Standard {
            nb_dice: 1,
            dice_faces: 8,
            striking_level: 0,
            potency_level: 0,
        },
    }
}
