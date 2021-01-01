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
        armor: Some(ArmorItem {
            info: ItemInfo {
                name: String::from("Scale mail"),
                bulk: 2,
            },
            ac_bonus: 3,
            dex_cap: 3,
            check_penalty: -2,
            ..Default::default()
        }),
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
