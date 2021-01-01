use crate::item::{armor::scale_mail, weapons::greatsword, HeadItem, ItemInfo, Loadout};

pub fn get_paladin_loadout() -> Loadout {
    let helmet = HeadItem {
        info: ItemInfo {
            name: String::from("Helmet"),
            bulk: 1,
        },
    };
    Loadout {
        head: Some(helmet),
        left_hand: Some(greatsword()),
        right_hand: None,
        armor: Some(scale_mail()),
    }
}
