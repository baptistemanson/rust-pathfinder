use dice::d20;

use crate::{
    character::Character,
    dice,
    item::weapon::{unarmed, CombatProperties, DamageType, WeaponItem},
    roll::Roll,
    world::World,
};
use crate::{
    item::traits::implementation::{attack_ability_modifier, deadly, striking},
    timeline::get_modifier,
};

use super::{find_target::find_first_conscious_enemy, Activity};

#[derive(Clone, Debug)]
pub struct Action<'a> {
    name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Action { name: "Attack" }
    }
}

struct AttackRollResults {
    value: i64,
    details: String,
    natural_20: bool,
}

pub struct DamageRollResults {
    pub value: i64,
    pub damage_type: DamageType,
    pub is_critical: bool,
    pub details: String,
}

fn compute_attack_roll(
    weapon: &WeaponItem,
    source: &Character,
    _target: &Character,
) -> AttackRollResults {
    // potency? => refactor because it is a bonus like any other
    let CombatProperties { potency_level, .. } = weapon.damage;
    let (item_bonus, item_detail) = if potency_level > 0 {
        (potency_level, format!(" + {} item", potency_level))
    } else {
        (0, String::new())
    };

    // strength or dexterity modifier
    let ability_score = if weapon.is_ranged {
        source.ability_score.dexterity
    } else {
        source.ability_score.strength
    };
    let ability_modifier = get_modifier(ability_score);
    let ability = if ability_modifier > 0 {
        format!(
            " + {} {}",
            ability_modifier,
            if weapon.is_ranged { "dex" } else { "str" }
        )
    } else {
        String::new()
    };

    // total
    // roll
    let roll = d20();

    let value = roll + ability_modifier + item_bonus;
    AttackRollResults {
        value,
        details: format!("1d20[{}]{}{}", roll, ability, item_detail),
        natural_20: roll == 20,
    }
}

// @todo eventually, there will be some deduction and addition here
fn compute_damage_roll(
    weapon: &WeaponItem,
    source: &Character,
    _target: &Character,
    is_critical: bool,
) -> DamageRollResults {
    // @todo add strength modifier
    let CombatProperties {
        dice_faces,
        nb_dice,
        ..
    } = weapon.damage;

    let striking_bonus = striking(weapon);
    let ability_modifier = attack_ability_modifier(weapon, source);

    let mut pre_crit = Roll::new(nb_dice + striking_bonus, dice_faces, 0) + ability_modifier;
    let mut post_crit = deadly(weapon, is_critical);
    let total = pre_crit.resolve() * if is_critical { 2 } else { 1 } + post_crit.resolve();

    DamageRollResults {
        value: total,
        damage_type: weapon.damage.damage_type, // because I turned it to a copy type... no prob bob
        is_critical,
        details: format!(
            "{crit}{precrit}{postcrit} = {total} dmg",
            crit = if is_critical { "critical 2x" } else { "" },
            precrit = pre_crit.get_summary(),
            postcrit = post_crit.get_summary(),
            total = total
        ),
    }
}

fn compute_ac(target: &Character) -> i64 {
    let armor_bonus = if let Some(armor) = &target.loadout.armor {
        armor.ac_bonus
    } else {
        0
    };
    10 + armor_bonus
}
impl<'a> Activity for Action<'a> {
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        dice::d20()
    }

    fn resolve<'lworld>(&self, source: &Character, world: &mut World) {
        let target_id = find_first_conscious_enemy(&source.party, world);
        match target_id {
            None => {
                return;
            }
            Some(id) => {
                let target: &mut Character = world.get_mut_character(&id);
                let f = &unarmed();
                let weapon = match (&source.loadout.left_hand, &source.loadout.right_hand) {
                    (_, Some(ref w)) => w, // @todo check the rules on lefty/righty rules.
                    (Some(ref w), None) => w,
                    (None, None) => f,
                };

                let attack_roll = compute_attack_roll(weapon, source, target);
                let ac_bonus = compute_ac(target);
                if ac_bonus > attack_roll.value {
                    println!(
                        "\t{} missed {} with {} ({} = {} vs {} AC)",
                        source.name,
                        target.name,
                        weapon.info.name,
                        attack_roll.details,
                        attack_roll.value,
                        ac_bonus
                    );
                    return;
                }
                // p278 critical hits
                let is_critical = attack_roll.natural_20 || (attack_roll.value - ac_bonus) >= 10;
                println!(
                    "\t{} {}hits {} with {} ({} = {} vs {} AC)",
                    source.name,
                    if is_critical { "critically " } else { "" },
                    target.name,
                    weapon.info.name,
                    attack_roll.details,
                    attack_roll.value,
                    ac_bonus
                );
                let dmg_result = compute_damage_roll(weapon, source, target, is_critical);
                let verb = match dmg_result.damage_type {
                    DamageType::Bludgeoning => "was bludgeoned for",
                    DamageType::Piercing => "was pierced for",
                    DamageType::Slashing => "was slashed for",
                };
                println!(
                    "\t{} {} {} damage ({})",
                    target.name, verb, dmg_result.value, dmg_result.details
                );
                (*target).sub_hp(dmg_result.value);
            }
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
