use crate::character::Character;
use crate::timeline::get_modifier;
use crate::character::bonus::{ResolvedBonus};
use crate::item::weapon::WeaponItem;
use crate::item::traits::Trait;
use crate::dice;

fn no_bonus() -> ResolvedBonus {
    ResolvedBonus{
        value: 0,
        details: String::from(""),
    }
}
pub fn deadly(weapon:&WeaponItem, is_critical: bool) -> ResolvedBonus {
    if is_critical && weapon.info.traits.contains(Trait::DeadlyD10 | Trait::DeadlyD8 | Trait::DeadlyD6) {
        // p282
        let deadly_flag = weapon.info.traits & (Trait::DeadlyD10 | Trait::DeadlyD8 | Trait::DeadlyD6);
        let size_to_roll = if deadly_flag == Trait::DeadlyD10 { 10 } else if deadly_flag == Trait::DeadlyD8 {8} else {6}; 

        let nb_to_roll = match weapon.damage.striking_level {
            0 => 1,
            1 => 1,
            2 => 2,
            _ => 3,
        };
        let value = nb_to_roll * dice::dx(size_to_roll);
        ResolvedBonus{
            value,
            details: format!(" + {}d{} {} deadly", nb_to_roll, size_to_roll, value),
        }
    } else {
        no_bonus()
    }
}


pub fn striking(weapon:&WeaponItem) -> ResolvedBonus {
    if weapon.damage.striking_level > 0 {ResolvedBonus{value: weapon.damage.striking_level, details: format!("+{}", weapon.damage.striking_level)}} else
    {no_bonus()}
}

pub fn ability(weapon:&WeaponItem, character: &Character) -> ResolvedBonus {
    if !weapon.is_ranged {
        let str_mod = get_modifier(character.ability_score.strength);
        ResolvedBonus{value: str_mod, details: format!(" + {} str", str_mod)}
    } else {no_bonus()}
}