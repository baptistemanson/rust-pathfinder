use super::{DamageFormula, ItemInfo, WeaponItem};

// p280 greatsword
pub fn greatsword() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            name: String::from("Greatsword +1"),
            bulk: 2,
        },
        is_two_hands: true,
        damage: DamageFormula::ClassicDamageFormula {
            nb_dice: 1,
            bonus: 1,
            dice_faces: 12,
        },
    }
}

// p280 fist
pub fn fist() -> WeaponItem {
    WeaponItem {
        info: ItemInfo {
            bulk: 0,
            name: String::from("Fist"),
        },
        is_two_hands: false,
        damage: DamageFormula::ClassicDamageFormula {
            nb_dice: 1,
            bonus: 0,
            dice_faces: 4,
        },
    }
}
