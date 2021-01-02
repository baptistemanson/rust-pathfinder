use crate::item::{armor::scale_mail, weapons::greatsword, HeadItem, ItemInfo, Loadout};

pub fn det_default_loadout() -> Loadout {
    let helmet = HeadItem {
        info: ItemInfo {
            name: String::from("Helmet"),
            bulk: 1,
        },
    };
    Loadout {
        head: Some(helmet),
        left_hand: None,
        right_hand: Some(greatsword()),
        armor: Some(scale_mail()),
    }
}
