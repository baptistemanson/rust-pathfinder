use super::{ArmorItem, ItemInfo};

pub fn leather() -> ArmorItem {
    ArmorItem {
        info: ItemInfo {
            name: String::from("Leather"),
            bulk: 1,
        },
        ac_bonus: 1,
        dex_cap: 4,
        check_penalty: -1,
        speed_penalty: 0,
        min_strength: 10,
    }
}

pub fn scale_mail() -> ArmorItem {
    ArmorItem {
        info: ItemInfo {
            name: String::from("Scale mail"),
            bulk: 2,
        },
        ac_bonus: 3,
        dex_cap: 3,
        check_penalty: -2,
        speed_penalty: 0,
        min_strength: 12,
    }
}
