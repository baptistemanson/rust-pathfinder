use armor::scale_mail;

use crate::item::weapons::greatsword;
use crate::item::*;

pub fn get_paladin_loadout() -> Slots {
    let helmet = HeadItem {
        info: ItemInfo {
            name: String::from("Helmet"),
            bulk: 1,
        },
    };
    Slots {
        head: Some(helmet),
        left_hand: Some(greatsword()),
        right_hand: None,
        armor: Some(scale_mail()),
    }
}

// Slots
#[derive(Clone, Debug)]
pub struct Slots {
    pub left_hand: Option<WeaponItem>,
    pub right_hand: Option<WeaponItem>,
    pub head: Option<HeadItem>,
    pub armor: Option<ArmorItem>,
}
