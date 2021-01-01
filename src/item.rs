pub mod armor;
pub mod weapons;

use super::dice;

pub trait Item {
    fn get_info(&self) -> &ItemInfo;
}

#[derive(Clone, Debug)]
pub struct ItemInfo {
    pub name: String,
    pub bulk: i64,
}

// will be able to generalize
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DamageFormula {
    ClassicDamageFormula {
        dice_faces: i64,
        nb_dice: i64,
        bonus: i64,
    },
}

pub struct DamageRollResults {
    pub value: i64,
    pub is_critical: bool,
    pub details: String,
}

impl DamageFormula {
    pub fn roll(&self, is_critical: bool) -> DamageRollResults {
        match self {
            DamageFormula::ClassicDamageFormula {
                dice_faces,
                nb_dice,
                bonus,
            } => {
                let roll = dice::dx(*dice_faces);
                let value = (roll * nb_dice + bonus) * if is_critical { 2 } else { 1 };
                DamageRollResults {
                    value: value,
                    is_critical,
                    details: if is_critical {
                        format!(
                            "Critical 2x [{} x d{} + {}] = {}",
                            nb_dice, dice_faces, bonus, value
                        )
                    } else {
                        format!("{}d{} + {} = {}", nb_dice, dice_faces, bonus, value)
                    },
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct WeaponItem {
    pub info: ItemInfo,
    pub damage: DamageFormula,
    pub is_two_hands: bool,
}

impl Item for WeaponItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Clone, Debug)]
pub struct ArmorItem {
    pub info: ItemInfo,
    pub ac_bonus: i64,
    pub dex_cap: i64,
    pub check_penalty: i64,
    pub speed_penalty: i64,
    pub min_strength: i64,
}

impl Item for ArmorItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Clone, Debug)]
pub struct HeadItem {
    pub info: ItemInfo,
}

#[derive(Clone, Debug)]
pub struct Loadout {
    pub left_hand: Option<WeaponItem>,
    pub right_hand: Option<WeaponItem>,
    pub head: Option<HeadItem>,
    pub armor: Option<ArmorItem>,
}
