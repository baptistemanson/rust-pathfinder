use dice::d20;

use crate::item::DamageRollResults;
use crate::{character::Character, dice, world::World};
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
    natural_20: bool,
}
fn compute_attack_roll(source: &Character, _target: &Character) -> AttackRoll {
    let roll = d20();
    AttackRoll {
        value: roll + get_modifier(source.ability_score.strength),
        natural_20: roll == 20,
    }
}

// @todo eventually, there will be some deduction and addition here
fn compute_damage_roll(
    source: &Character,
    _target: &Character,
    is_critical: bool,
) -> DamageRollResults {
    let f = fist();
    let weapon = match (&source.slots.left_hand, &source.slots.right_hand) {
        (_, Some(w)) => w, // @todo check the rules on lefty/righty rules.
        (Some(w), None) => w,
        (None, None) => &f,
    };
    weapon.damage.roll(is_critical)
}

fn compute_ac(target: &Character) -> i64 {
    let armor_bonus = if let Some(armor) = &target.slots.armor {
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
                let attack_roll = compute_attack_roll(source, target);
                let ac_bonus = compute_ac(target);
                if ac_bonus > attack_roll.value {
                    println!(
                        "\t{} missed {} ({} vs {} AC)",
                        source.name, target.name, attack_roll.value, ac_bonus
                    );
                    return;
                }
                // p278 critical hits
                let is_critical = attack_roll.natural_20 || (attack_roll.value - ac_bonus) >= 10;
                println!(
                    "\t{} {}hits {} ({} vs {} AC)",
                    source.name,
                    if is_critical { "critically " } else { "" },
                    target.name,
                    attack_roll.value,
                    ac_bonus
                );
                let dmg_result = compute_damage_roll(source, target, is_critical);
                println!(
                    "\t{} causes {} for {} damage ({})",
                    source.name, target.name, dmg_result.value, dmg_result.details
                );
                (*target).sub_hp(dmg_result.value);
            }
        }
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
