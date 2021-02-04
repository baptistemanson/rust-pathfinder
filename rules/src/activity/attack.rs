use dice::Roll;

use crate::{
    character::Character,
    fact,
    item::{
        weapon::{CombatProperties, DamageType, WeaponItem},
        AnyItem,
    },
    rules::Rule,
    status::StatusType,
    utils::get_armor,
    world::World,
};
use crate::{timeline::get_modifier, utils::get_active_weapon};

use super::{find_target::find_first_conscious_enemy, Activity};

#[derive(Clone, Debug)]
pub struct Action;

impl Action {
    pub fn new() -> Self {
        Self {}
    }
}

impl Activity for Action {
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        Roll::d("", 1, 20).roll()
    }

    fn resolve<'lworld>(&mut self, source: &Character, world: &mut World, facts: &mut fact::Facts) {
        let target_id = find_first_conscious_enemy(&source.party, world);
        match target_id {
            None => {
                return;
            }
            Some(id) => {
                let target: &Character = world.get_character(&id);
                let weapon = get_active_weapon(&source, world);

                let attack_roll = compute_attack_roll(&weapon, source, target);
                let ac_bonus = compute_ac(target, world);

                if ac_bonus > attack_roll.value {
                    facts.info(&format!(
                        "\t{} missed {} with {} ({} = {} vs {} AC)",
                        source.name,
                        target.name,
                        weapon.info.name,
                        attack_roll.details,
                        attack_roll.value,
                        ac_bonus
                    ));
                    return;
                }
                // p278 critical hits
                let is_critical = attack_roll.natural_20 || (attack_roll.value - ac_bonus) >= 10;
                facts.info(&format!(
                    "\t{} {}hits {} with {} ({} = {} vs {} AC)",
                    source.name,
                    if is_critical { "critically " } else { "" },
                    target.name,
                    weapon.info.name,
                    attack_roll.details,
                    attack_roll.value,
                    ac_bonus
                ));
                let dmg = compute_damage_roll(&weapon, source, target, world, is_critical);
                let verb = match dmg.damage_type {
                    DamageType::Bludgeoning => "was bludgeoned for",
                    DamageType::Piercing => "was pierced for",
                    DamageType::Slashing => "was slashed for",
                };
                facts.info(&format!(
                    "\t{} {} {} damage ({})",
                    target.name, verb, dmg.value, dmg.details,
                ));

                // apply damage and statuses and loosing objects and...
                let target: &mut Character = world.get_mut_character(&id); // could this be avoided? maybe
                (*target).sub_hp(dmg.value);
            }
        }
    }

    fn get_name(&self) -> &str {
        "Attack"
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
    let roll = Roll::d("", 1, 20);
    // strength or dexterity modifier
    let ability_score = if weapon.is_ranged {
        Roll::flat("dex", get_modifier(source.ability_score.dexterity))
    } else {
        Roll::flat("str", get_modifier(source.ability_score.strength))
    };

    // @todo move this to a rule.
    let status_bonus = if source.has_status(StatusType::Bless) {
        Roll::flat("bless", 1)
    } else {
        Roll::default()
    };
    let item_bonus = Roll::default();

    // total
    let mut total = roll + ability_score + item_bonus + status_bonus;
    AttackRollResults {
        value: total.resolve(),
        details: total.to_string(),
        natural_20: total.resolve() == 20,
    }
}

fn compute_damage_roll(
    weapon: &WeaponItem,
    source: &Character,
    target: &Character,
    world: &World,
    is_critical: bool,
) -> DamageRollResults {
    let CombatProperties {
        dice_faces,
        nb_dice,
        ..
    } = weapon.damage;

    let mut rules = vec![Rule::StrengthModDamage];
    rules.extend(weapon.info.rules.clone());
    rules.extend(get_armor(target, world).info.rules.clone());

    let mut pre_crit_roll = Roll::d("weapon", nb_dice, dice_faces);
    pre_crit_roll = world
        .rules
        .dmg_pre_crit(&rules, pre_crit_roll, source, world);

    let mut post_crit_roll =
        world
            .rules
            .dmg_post_crit(&weapon.info.rules, Roll::default(), source, world);

    let total =
        pre_crit_roll.resolve() * if is_critical { 2 } else { 1 } + post_crit_roll.resolve();

    let pc_str = post_crit_roll.to_string();

    DamageRollResults {
        value: total,
        damage_type: weapon.damage.damage_type, // because I turned it to a copy type... no prob bob
        is_critical,
        details: if is_critical {
            format!(
                "critical 2x({precrit}) + {postcrit} = {total} dmg",
                precrit = pre_crit_roll.to_string(),
                postcrit = post_crit_roll.to_string(),
                total = total
            )
        } else {
            format!(
                "{precrit}{sep}{postcrit} = {total} dmg",
                precrit = pre_crit_roll.to_string(),
                sep = if pc_str != "" { " + " } else { "" },
                postcrit = pc_str,
                total = total
            )
        },
    }
}

fn compute_ac(target: &Character, world: &World) -> i64 {
    let ac_bonus_armor = {
        if let Some(armor_id) = &target.loadout.armor {
            let item = world.items.get(armor_id).expect("lost armor");
            match item {
                AnyItem::ArmorItem(a) => a.ac_bonus,
                _ => panic!("do not know"),
            }
        } else {
            0
        }
    };
    10 + ac_bonus_armor
}
