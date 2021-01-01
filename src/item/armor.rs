use super::{ArmorItem, ItemInfo};

pub fn leather() -> ArmorItem {
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
