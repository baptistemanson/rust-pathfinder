use crate::item::*;

pub fn get_paladin_loadout() -> Slots {
    let zweihander = TwoHandItem {
        info: ItemInfo {
            name: String::from("Zweihander"),
            bulk: 10,
        },
        damage: DamageFormula::ClassicDamageFormula{nb_dice: 1, bonus:1, dice_faces: 8}
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

// Slots
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum HandSlot {
    SingleHands(Option<OneHandItem>, Option<OneHandItem>),
    TwoHands(TwoHandItem),
}

#[derive(Clone, Debug)]
pub struct Slots {
    pub hands: HandSlot,
    pub face: Option<FaceItem>,
}
