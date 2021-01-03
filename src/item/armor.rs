use super::{traits::none, GameItem, HeadItem, ItemInfo};

#[derive(Clone, Debug)]
pub struct ArmorItem {
    pub info: ItemInfo,
    pub ac_bonus: i64,
    pub dex_cap: i64,
    pub check_penalty: i64,
    pub speed_penalty: i64,
    pub min_strength: i64,
}

impl GameItem for ArmorItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

pub fn leather() -> ArmorItem {
    ArmorItem {
        info: ItemInfo::new("Leather", 1, none()),
        ac_bonus: 1,
        dex_cap: 4,
        check_penalty: -1,
        speed_penalty: 0,
        min_strength: 10,
    }
}

pub fn scale_mail() -> ArmorItem {
    ArmorItem {
        info: ItemInfo::new("Scale Mail", 2, none()),
        ac_bonus: 3,
        dex_cap: 3,
        check_penalty: -2,
        speed_penalty: 0,
        min_strength: 12,
    }
}

pub fn helmet() -> HeadItem {
    HeadItem {
        info: ItemInfo::new("Helmet", 1, none()),
    }
}
