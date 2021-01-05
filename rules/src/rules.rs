use self::{
    deadly::DeadlyRule, finesse::FinessRule, passthrough::Passthrough, propulsive::PropulsiveRule,
    str_mod_damage::StrengthModDamageRule, striking::StrikingRule,
};
use crate::world::World;
use crate::{character::Character, roll::Roll};
use std::collections::HashMap;

mod deadly;
mod finesse;
mod passthrough;
mod propulsive;
mod str_mod_damage;
mod striking;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Rule {
    Propulsive,
    Finesse,
    Striking(usize),
    Deadly(usize),
    StrengthModDamage,
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Monk,
    Orc,
    ColdIron,
    Adamantine,
    Silver,
}

// pub enum PrevActionResult {
//     None,
//     AttackMiss,
//     AttackHit,
// }

pub trait RuleImplementation {
    fn dmg_pre_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        r
    }
    fn dmg_post_crit(&self, r: Roll, _: &Character, _: &World) -> Roll {
        r
    }
    fn dmg_reduction(&self, r: Roll, _: &Character, _: &World) -> Roll {
        r
    }
}

pub struct RuleBook {
    pub rules: HashMap<Rule, Box<dyn RuleImplementation>>,
}

impl RuleBook {
    pub fn new() -> RuleBook {
        RuleBook {
            rules: HashMap::new(),
        }
    }
    pub fn load_rule(&mut self, r: Rule, ri: Box<dyn RuleImplementation>) {
        self.rules.insert(r, ri);
    }

    pub fn load_rules(&mut self) {
        // basic rules
        self.load_rule(Rule::StrengthModDamage, Box::new(StrengthModDamageRule {}));

        // weapon traits
        self.load_rule(Rule::Finesse, Box::new(FinessRule {}));
        self.load_rule(Rule::Propulsive, Box::new(PropulsiveRule {}));
        self.load_rule(Rule::Striking(1), Box::new(StrikingRule { level: 1 }));
        self.load_rule(Rule::Striking(2), Box::new(StrikingRule { level: 2 }));
        self.load_rule(Rule::Striking(3), Box::new(StrikingRule { level: 3 }));
        self.load_rule(Rule::Deadly(1), Box::new(DeadlyRule { die: 1 }));
        self.load_rule(Rule::Deadly(2), Box::new(DeadlyRule { die: 2 }));
        self.load_rule(Rule::Deadly(3), Box::new(DeadlyRule { die: 3 }));

        // races
        self.load_rule(Rule::Dwarf, Box::new(Passthrough {}));
        self.load_rule(Rule::Elf, Box::new(Passthrough {}));
        self.load_rule(Rule::Gnome, Box::new(Passthrough {}));
        self.load_rule(Rule::Goblin, Box::new(Passthrough {}));
        self.load_rule(Rule::Monk, Box::new(Passthrough {}));
        self.load_rule(Rule::Orc, Box::new(Passthrough {}));

        // weapon material
        self.load_rule(Rule::Adamantine, Box::new(Passthrough {}));
        self.load_rule(Rule::ColdIron, Box::new(Passthrough {}));
        self.load_rule(Rule::Silver, Box::new(Passthrough {}));
    }

    // @todo group in a CombatContext all the fields, add is_critical, prev_action_result, action_nb
    pub fn dmg_pre_crit(
        &self,
        active_rules: &Vec<Rule>,
        mut roll: Roll,
        character: &Character,
        world: &World,
    ) -> Roll {
        for rule in active_rules {
            let maybe_rule = self.rules.get(&rule);
            match maybe_rule {
                None => eprintln!("missing rule from the rulebook"),
                Some(rule_impl) => roll = rule_impl.dmg_pre_crit(roll, character, world),
            }
        }
        roll
    }

    pub fn dmg_post_crit(
        &self,
        active_rules: &Vec<Rule>,
        mut roll: Roll,
        character: &Character,
        world: &World,
    ) -> Roll {
        for rule in active_rules {
            let maybe_rule = self.rules.get(&rule);
            match maybe_rule {
                None => eprintln!("missing rule from the rulebook"),
                Some(rule_impl) => roll = rule_impl.dmg_post_crit(roll, character, world),
            }
        }
        roll
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::Character;

    #[test]
    fn no_rules() {
        let c = Character::default(); // char should be registered, but hey
        let w = World::new();

        let r = RuleBook::new();
        assert_eq!(
            r.dmg_pre_crit(&vec![], Roll::new(1, 6, 2), &c, &w),
            Roll::new(1, 6, 2)
        );
    }

    #[test]
    fn finesse() {
        let mut c = Character::default(); // char should be registered, but hey
        let w = World::new();
        let mut r = RuleBook::new();

        r.load_rule(Rule::Finesse, Box::new(FinessRule {}));
        let active_rules = vec![Rule::Finesse];
        c.ability_score.strength = 12;
        c.ability_score.dexterity = 18;
        let roll = Roll::new(1, 6, 1); // bonus of str 1 already baked.
        assert_eq!(
            r.dmg_pre_crit(&active_rules, roll, &c, &w),
            Roll::new(1, 6, 4)
        );

        c.ability_score.dexterity = 8;
        let roll = Roll::new(1, 6, 1);
        assert_eq!(
            r.dmg_pre_crit(&active_rules, roll, &c, &w),
            Roll::new(1, 6, 1)
        )
    }
}
