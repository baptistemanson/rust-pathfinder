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

pub fn striking(weapon: &WeaponItem) -> usize {
    if weapon.damage.striking_level > 0 {
        1
    } else {
        0
    }
}

pub fn attack_ability_modifier(weapon: &WeaponItem, character: &Character) -> Roll {
    if !weapon.is_ranged {
        let str_mod = get_modifier(character.ability_score.strength);
        // finesse
        let dex_mod = get_modifier(character.ability_score.dexterity);
        if str_mod <= dex_mod && weapon.info.traits.contains(Trait::Finesse) {
            Roll::new(0, 0, dex_mod)
        } else {
            Roll::new(0, 0, str_mod)
        }
    } else {
        if weapon.info.traits.contains(Trait::Propulsive) {
            let str_mod = get_modifier(character.ability_score.strength);
            //@todo check rounding rules for propulsive
            if str_mod >= 0 {
                Roll::new(0, 0, str_mod / 2)
            } else {
                Roll::new(0, 0, str_mod)
            }
        } else {
            Roll::default()
        }
    }
}
