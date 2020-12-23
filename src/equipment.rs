use serde::{Deserialize, Serialize};
// Try to remodel this, but reversed.

trait GameObject {
    fn get_info(&self) -> &ItemInfo;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemInfo {
    name: String,
    bulk: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OneHandItem {
    info: ItemInfo,
}

impl GameObject for OneHandItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

impl GameObject for TwoHandItem {
    fn get_info(&self) -> &ItemInfo {
        &self.info
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TwoHandItem {
    info: ItemInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FaceItem {
    info: ItemInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Item {
    FaceItem(FaceItem),
    TwoHandItem(TwoHandItem),
    OneHandItem(OneHandItem),
}

type Stash = Vec<Item>;

pub fn get_default_equipment() -> Slots {
    let zweihander = TwoHandItem {
        info: ItemInfo {
            name: String::from("Zweihander"),
            bulk: 10,
        },
    };
    let death_goggle = FaceItem {
        info: ItemInfo {
            name: String::from("Death Google"),
            bulk: 1,
        },
    };
    Slots {
        face: Some(death_goggle),
        hands: HandSlot::TwoHands(zweihander),
    }
}

pub fn get_backpack() -> Stash {
    let zweihander = TwoHandItem {
        info: ItemInfo {
            name: String::from("Zweihander"),
            bulk: 10,
        },
    };
    let death_goggle = FaceItem {
        info: ItemInfo {
            name: String::from("Death Google"),
            bulk: 1,
        },
    };
    vec![Item::TwoHandItem(zweihander), Item::FaceItem(death_goggle)]
}

pub fn view_stash(stash: &Stash) {
    println!("what is in the backpack huh?");
    for item in stash {
        match item {
            Item::TwoHandItem(e) => println!("A two hand item! {:?}", e),
            Item::FaceItem(e) => println!("An unknwon thing! {:?}", e),
            _ => println!("something else"),
        }
    }
}

// Slots
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HandSlot {
    SingleHands(Option<OneHandItem>, Option<OneHandItem>),
    TwoHands(TwoHandItem),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Slots {
    hands: HandSlot,
    face: Option<FaceItem>,
}
