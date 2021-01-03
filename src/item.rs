pub mod armor;
pub mod traits;
pub mod weapon;

use traits::TraitSet;

use crate::roll::Roll;

use self::{
    armor::{scale_mail, ArmorItem},
    traits::none,
    weapon::{greatsword, WeaponItem},
};

pub trait GameItem {
    fn get_info(&self) -> &ItemInfo;
    fn get_bonuses(&self) -> Vec<Roll> {
        vec![]
    }
}
#[derive(Clone, Debug)]
pub struct ItemInfo {
    pub name: String,
    pub bulk: i64,
    pub traits: TraitSet,
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

impl Default for Loadout {
    fn default() -> Loadout {
        let helmet = HeadItem {
            info: ItemInfo {
                name: String::from("Helmet"),
                bulk: 1,
                traits: none(),
            },
        };
        Loadout {
            head: Some(helmet),
            left_hand: None,
            right_hand: Some(greatsword()),
            armor: Some(scale_mail()),
        }
    }
}
