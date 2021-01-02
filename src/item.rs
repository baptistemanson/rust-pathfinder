pub mod armor;
pub mod itemtrait;
pub mod weapon;

use crate::character::bonus::Bonus;

use self::{armor::ArmorItem, weapon::WeaponItem};

pub trait GameItem {
    fn get_info(&self) -> &ItemInfo;
    fn get_bonuses(&self) -> Vec<Bonus> {
        vec![]
    }
}
#[derive(Clone, Debug)]
pub struct ItemInfo {
    pub name: String,
    pub bulk: i64,
    pub traits: u64,
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
    pub left_hand: Option<WeaponItem>,
    pub right_hand: Option<WeaponItem>,
    pub head: Option<HeadItem>,
    pub armor: Option<ArmorItem>,
}

impl Loadout {
    pub fn as_vec<'a>(&'a self) -> Vec<Box<dyn GameItem>> {
        let mut loadout: Vec<Box<dyn GameItem>> = Vec::new();
        if let Some(i) = self.left_hand.clone() {
            loadout.push(Box::new(i))
        };
        if let Some(i) = self.right_hand.clone() {
            loadout.push(Box::new(i))
        };
        if let Some(i) = self.head.clone() {
            loadout.push(Box::new(i))
        };
        if let Some(i) = self.armor.clone() {
            loadout.push(Box::new(i))
        };

        loadout
    }
}
