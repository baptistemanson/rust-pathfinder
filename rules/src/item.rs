pub mod armor;
pub mod weapon;

use weapon::WeaponItem;

use crate::rules::Rule;

use self::armor::ArmorItem;

pub type ItemId = String;

pub trait GameItem {
    fn get_info(&self) -> &ItemInfo;
}

pub enum AnyItem {
    WeaponItem(WeaponItem),
    ArmorItem(ArmorItem),
    #[allow(dead_code)]
    HeadItem(HeadItem),
}

#[derive(Clone, Debug)]
pub struct ItemInfo {
    pub id: String,
    pub name: String,
    pub bulk: i64,
    pub rules: Vec<Rule>,
}

impl ItemInfo {
    pub fn new(name: &str, bulk: i64, rules: Vec<Rule>) -> Self {
        let name = String::from(name);
        ItemInfo {
            id: format!("{}-{}", name, snowflake::ProcessUniqueId::new()),
            name,
            bulk,
            rules,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HeadItem {
    pub info: ItemInfo,
}

impl GameItem for HeadItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Clone, Debug)]
pub struct Loadout {
    pub left_hand: Option<ItemId>,
    pub right_hand: Option<ItemId>,
    pub head: Option<ItemId>,
    pub armor: Option<ItemId>,
}

impl Default for Loadout {
    fn default() -> Loadout {
        Loadout {
            head: None,
            left_hand: None,
            right_hand: None,
            armor: None,
        }
    }
}
