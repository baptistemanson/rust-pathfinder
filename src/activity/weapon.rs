use dice::d20;

use crate::{
    character::Character,
    dice,
    item::weapons::{CombatProperties, DamageRollResults, WeaponItem},
    world::World,
};
use crate::{item::weapons::fist, timeline::get_modifier};

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

struct AttackRoll {
    value: i64,
    details: String,
    natural_20: bool,
}
fn compute_attack_roll(weapon: &WeaponItem, source: &Character, _target: &Character) -> AttackRoll {
    // roll
    let roll = d20();

    // potency? => refactor because it is a bonus like any other
    let (item_bonus, item_detail) =
        if let CombatProperties::Standard { potency_level, .. } = weapon.damage {
            (
                potency_level,
                if potency_level > 0 {
                    format!(" + {} item bonus", potency_level)
                } else {
                    String::new()
                },
            )
        } else {
            (0, String::new())
        };

    // strength
    let strength_modifier = get_modifier(source.ability_score.strength);
    let strength_modifier_details = if strength_modifier > 0 {
        format!(" + {}", strength_modifier)
    } else {
        String::new()
    };

    // total
    let value = roll + strength_modifier + item_bonus;
    AttackRoll {
        value,
        details: format!(
            "⚅ {}{}{} = {}",
            roll, strength_modifier_details, item_detail, value
        ),
        natural_20: roll == 20,
    }
}

// @todo eventually, there will be some deduction and addition here
fn compute_damage_roll(
    weapon: &WeaponItem,
    _source: &Character,
    _target: &Character,
    is_critical: bool,
) -> DamageRollResults {
    weapon.damage.roll(is_critical)
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
                let f = &fist();
                let weapon = match (&source.loadout.left_hand, &source.loadout.right_hand) {
                    (_, Some(ref w)) => w, // @todo check the rules on lefty/righty rules.
                    (Some(ref w), None) => w,
                    (None, None) => f,
                };
                let attack_roll = compute_attack_roll(weapon, source, target);
                let ac_bonus = compute_ac(target);
                if ac_bonus > attack_roll.value {
                    println!(
                        "\t{} missed {} with {} ({} vs {} AC)",
                        source.name, target.name, weapon.info.name, attack_roll.details, ac_bonus
                    );
                    return;
                }
                // p278 critical hits
                let is_critical = attack_roll.natural_20 || (attack_roll.value - ac_bonus) >= 10;
                println!(
                    "\t{} {}hits {} with {} ({} vs {} AC)",
                    source.name,
                    if is_critical { "critically " } else { "" },
                    target.name,
                    weapon.info.name,
                    attack_roll.value,
                    ac_bonus
                );
                let dmg_result = compute_damage_roll(weapon, source, target, is_critical);
                println!(
                    "\t{} takes {} damage ({})",
                    target.name, dmg_result.value, dmg_result.details
                );
                (*target).sub_hp(dmg_result.value);
            }
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
