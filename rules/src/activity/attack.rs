use dice::Roll;

use crate::{
    character::Character,
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
pub struct Action<'a> {
    name: &'a str,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Action { name: "Attack" }
    }
}

impl<'a> Activity for Action<'a> {
    fn ai_playing_value(&self, _character: &Character, _context: &World) -> i64 {
        Roll::from("1d20").roll()
    }

    fn resolve<'lworld>(&self, source: &Character, world: &mut World) {
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
                let dmg_result = compute_damage_roll(&weapon, source, target, world, is_critical);
                let verb = match dmg_result.damage_type {
                    DamageType::Bludgeoning => "was bludgeoned for",
                    DamageType::Piercing => "was pierced for",
                    DamageType::Slashing => "was slashed for",
                };
                println!(
                    "\t{} {} {} damage ({})",
                    target.name, verb, dmg_result.value, dmg_result.details
                );

                // apply damage and statuses and loosing objects and...
                // create struct for those changes and apply it to each object of the collection in the main
                let target: &mut Character = world.get_mut_character(&id); // could this be avoided? maybe
                (*target).sub_hp(dmg_result.value);
            }
        }
    }

    fn get_name(&self) -> &str {
        self.name
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
    let item_bonus = Roll::flat("item", 0);

    // strength or dexterity modifier
    let ability_score = if weapon.is_ranged {
        Roll::flat("dex", get_modifier(source.ability_score.dexterity))
    } else {
        Roll::flat("str", get_modifier(source.ability_score.strength))
    };

    let status_bonus = if source.has_status(StatusType::Bless) {
        Roll::flat("bless", 1)
    } else {
        Roll::new("", 0, 0, 0)
    };
    // roll
    let roll = Roll::from("1d20");
    // total
    let mut total = roll + ability_score + item_bonus + status_bonus;
    AttackRollResults {
        value: total.resolve(),
        details: total.get_details(),
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

    let mut pre_crit_roll = Roll::d("weapon", nb_dice, dice_faces);

    let mut all_rules = vec![Rule::StrengthModDamage];
    all_rules.extend(weapon.info.rules.clone());
    // i dont think it will do anything here?
    let defender_armor = get_armor(target, world);
    all_rules.extend(defender_armor.info.rules);

    pre_crit_roll = world
        .rules
        .dmg_pre_crit(&all_rules, pre_crit_roll, source, world);

    let mut post_crit = world.rules.dmg_post_crit(
        &weapon.info.rules,
        Roll::d("placeholder", 0, 0),
        source,
        world,
    );

    let total = pre_crit_roll.resolve() * if is_critical { 2 } else { 1 } + post_crit.resolve();

    DamageRollResults {
        value: total,
        damage_type: weapon.damage.damage_type, // because I turned it to a copy type... no prob bob
        is_critical,
        details: format!(
            "{crit}{precrit} {postcrit} = {total} dmg",
            crit = if is_critical { "critical 2x" } else { "" },
            precrit = pre_crit_roll.get_details(),
            postcrit = post_crit.get_details(),
            total = total
        ),
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
