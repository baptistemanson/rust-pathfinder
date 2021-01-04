use crate::character::Character;
use crate::item::traits::Trait;
use crate::item::weapon::WeaponItem;
use crate::roll::Roll;
use crate::timeline::get_modifier;

pub fn deadly(weapon: &WeaponItem, is_critical: bool) -> Roll {
    if is_critical
        && weapon
            .info
            .traits
            .contains(Trait::DeadlyD10 | Trait::DeadlyD8 | Trait::DeadlyD6)
    {
        // p282
        let deadly_flag =
            weapon.info.traits & (Trait::DeadlyD10 | Trait::DeadlyD8 | Trait::DeadlyD6);
        let size_to_roll = if deadly_flag == Trait::DeadlyD10 {
            10
        } else if deadly_flag == Trait::DeadlyD8 {
            8
        } else {
            6
        };

        let nb_to_roll = match weapon.damage.striking_level {
            0 => 1,
            1 => 1,
            2 => 2,
            _ => 3,
        };
        Roll::new(nb_to_roll, size_to_roll, 0)
    } else {
        Roll::default()
    }
}
