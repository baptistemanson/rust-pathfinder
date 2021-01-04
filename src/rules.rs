use std::collections::HashMap;

use regex::internal::Char;

use crate::{character::Character, roll::Roll};
use crate::{timeline::get_modifier, world::World};

/*
per character
Rules {
    start_of_action(mut Character, Vec<Activities>, ) -> Vec<Activities>
    end_of_action(mut Character, ActionResult) -> nothing
    start_of
}

*/
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Rule {
    Propulsive,
    Finesse,
    Striking(usize),
}

// pub enum PrevActionResult {
//     None,
//     AttackMiss,
//     AttackHit,
// }

// Each time I have to find a new hook point for a rule, I will add it here I guess.
pub trait RuleImplementation {
    fn attack_ability_modifier(&self, r: Roll, c: &Character, w: &World) -> Roll {
        r
    }
}

struct FinessRule {}
impl RuleImplementation for FinessRule {
    fn attack_ability_modifier(&self, r: Roll, c: &Character, w: &World) -> Roll {
        let str_mod = get_modifier(c.ability_score.strength);
        let dex_mod = get_modifier(c.ability_score.dexterity);
        if str_mod < dex_mod {
            r + (dex_mod - str_mod) // replace str by dex
        } else {
            r
        }
    }
}
/*

*/
struct PropulsiveRule {}
impl RuleImplementation for PropulsiveRule {
    fn attack_ability_modifier(&self, r: Roll, c: &Character, w: &World) -> Roll {
        let str_mod = get_modifier(c.ability_score.strength);
        //@todo check rounding rules for propulsive
        if str_mod >= 0 {
            r + (str_mod / 2)
        } else {
            r + str_mod
        }
    }
}

struct StrikingRule {
    level: usize,
}

impl RuleImplementation for StrikingRule {
    fn attack_ability_modifier(&self, r: Roll, _: &Character, _: &World) -> Roll {
        let extra_die = if r.dices.len() > 0 {
            r.dices[0]
        } else {
            panic!("Striking cannot find the prev die")
        };
        r + Roll::new(self.level, extra_die, 0)
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
        self.load_rule(Rule::Finesse, Box::new(FinessRule {}));
        self.load_rule(Rule::Propulsive, Box::new(PropulsiveRule {}));
        // kinda cool. could also forward the rule to the function if it gets too messy
        self.load_rule(Rule::Striking(1), Box::new(StrikingRule { level: 1 }));
        self.load_rule(Rule::Striking(2), Box::new(StrikingRule { level: 2 }));
        self.load_rule(Rule::Striking(3), Box::new(StrikingRule { level: 3 }));
    }

    pub fn apply_attack_ability_modifier(
        &self,
        active_rules: &Vec<Rule>,
        mut roll: Roll,
        character: &Character,
        world: &World,
    ) -> Roll {
        for rule in active_rules {
            let maybe_rule = self.rules.get(&rule);
            match maybe_rule {
                None => eprintln!("missing rule"),
                Some(rule_impl) => roll = rule_impl.attack_ability_modifier(roll, character, world),
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
            r.apply_attack_ability_modifier(&vec![], Roll::new(1, 6, 2), &c, &w),
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
            r.apply_attack_ability_modifier(&active_rules, roll, &c, &w),
            Roll::new(1, 6, 4)
        );

        c.ability_score.dexterity = 8;
        let roll = Roll::new(1, 6, 1);
        assert_eq!(
            r.apply_attack_ability_modifier(&active_rules, roll, &c, &w),
            Roll::new(1, 6, 1)
        )
    }
}
