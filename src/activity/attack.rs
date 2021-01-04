use dice::d20;

use crate::timeline::get_modifier;
use crate::{
    character::Character,
    dice,
    item::{
        weapon::{unarmed, CombatProperties, DamageType, WeaponItem},
        AnyItem, Loadout,
    },
    roll::Roll,
    status::StatusType,
    world::World,
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

fn get_weapon_from_loadout(loadout: &Loadout, world: &World) -> WeaponItem {
    let id = match (&loadout.left_hand, &loadout.right_hand) {
        (_, Some(ref w)) => w, // @todo check the rules on lefty/righty rules.
        (Some(ref w), None) => w,
        (None, None) => "",
    };
    if id == "" {
        unarmed()
    } else {
        if let AnyItem::WeaponItem(item) = world.items.get(id).expect("cannot find weapon") {
            item.clone()
        } else {
            panic!("this is not a weapon")
        }
    }
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
                let target: &Character = world.get_character(&id);
                let weapon = get_weapon_from_loadout(&source.loadout, world);

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

    let (status_bonus, status_detail) = if source.has_status(StatusType::Bless) {
        (1, " + 1 status")
    } else {
        (0, "")
    };
    // roll
    let roll = d20();
    // total
    let value = roll + ability_modifier + item_bonus + status_bonus;
    AttackRollResults {
        value,
        details: format!("1d20[{}]{}{}{}", roll, ability, item_detail, status_detail),
        natural_20: roll == 20,
    }
}

// @todo eventually, there will be some deduction and addition here
fn compute_damage_roll(
    weapon: &WeaponItem,
    source: &Character,
    _target: &Character,
    world: &World,
    is_critical: bool,
) -> DamageRollResults {
    // @todo add strength modifier
    let CombatProperties {
        dice_faces,
        nb_dice,
        ..
    } = weapon.damage;

    //  let striking_bonus = striking(weapon);

    let mut pre_crit = Roll::new(nb_dice, dice_faces, 0);
    // rules
    pre_crit =
        world
            .rules
            .apply_attack_ability_modifier(&weapon.info.rules, pre_crit, source, world);

    let total = pre_crit.resolve() * if is_critical { 2 } else { 1 };

    DamageRollResults {
        value: total,
        damage_type: weapon.damage.damage_type, // because I turned it to a copy type... no prob bob
        is_critical,
        details: format!(
            "{crit}{precrit}{postcrit} = {total} dmg",
            crit = if is_critical { "critical 2x" } else { "" },
            precrit = pre_crit.get_summary(),
            postcrit = Roll::new(0, 0, 0).get_summary(),
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
