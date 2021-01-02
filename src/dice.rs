use regex::Regex;
use std::ops;

use rand::prelude::*;

// increasing dice size p279
#[allow(dead_code)]
pub fn increase_dice(x: i64) -> i64 {
    x + 2
}

pub fn dxx(x: i64, nb_dice: i64) -> i64 {
    let mut total = 0;
    for _ in 0..nb_dice {
        total += thread_rng().gen_range(1..=x)
    }
    total
}
pub fn dx(x: i64) -> i64 {
    thread_rng().gen_range(1..=x)
}
pub fn d20() -> i64 {
    thread_rng().gen_range(1..=20)
}

type Dice = i64;

/**
 */
pub struct Roll {
    pub dices: Vec<Dice>,
    pub flat_bonus: i64,
}

impl Roll {
    pub fn new(nb_dice: usize, face: i64, flat_bonus: i64) -> Self {
        Roll {
            dices: vec![face; nb_dice],
            flat_bonus,
        }
    }

    pub fn from(expr: &str) -> Self {
        let re = Regex::new(r"^(?P<nb>\d+)d(?P<faces>\d+)\+(?P<flat>\d+)$").unwrap();
        let mut nb_dice = 0;
        let mut faces = 0;
        let mut flat_bonus = 0;
        for cap in re.captures_iter(expr) {
            nb_dice = cap[1].parse::<usize>().unwrap();
            faces = cap[2].parse::<i64>().unwrap();
            flat_bonus = cap[3].parse::<i64>().unwrap();
        }
        Roll {
            dices: vec![faces; nb_dice],
            flat_bonus,
        }
    }

    // @todo doesnt handle different dice types.
    ///```
    ///use pathfinder::dice::Roll;
    ///let roll = Roll::new(2, 6, 3);
    ///assert_eq!(roll.get_summary(), "2d6+3");
    ///```
    pub fn get_summary(&self) -> String {
        match (self.dices.len(), self.flat_bonus) {
            (0, 0) => String::from(""),
            (0, x) => format!("+{}", x),
            (_, 0) => format!("{}d{}", self.dices.len(), self.dices[0]),
            _ => format!("{}d{}+{}", self.dices.len(), self.dices[0], self.flat_bonus),
        }
    }
    pub fn resolve(&self) -> i64 {
        let mut value = 0;
        for d in self.dices.iter() {
            value += dx(*d);
        }
        value += self.flat_bonus;
        value
    }
}

impl ops::Add<Roll> for Roll {
    type Output = Roll;

    fn add(self, rhs: Roll) -> Roll {
        let dices = [self.dices, rhs.dices].concat();
        let flat_bonus = self.flat_bonus + rhs.flat_bonus;
        Roll { dices, flat_bonus }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let roll = Roll::new(1, 6, 3);
        assert_eq!(roll.dices.len(), 1);
        assert_eq!(roll.dices[0], 6);
        assert_eq!(roll.flat_bonus, 3);
    }

    #[test]
    fn add() {
        let roll = Roll::new(1, 6, 3) + Roll::new(2, 3, 5);
        assert_eq!(roll.dices, vec![6, 3, 3]);
        assert_eq!(roll.flat_bonus, 8);
    }

    #[test]
    fn resolve() {
        let roll = Roll::new(6, 1, 3);
        assert_eq!(roll.resolve(), 9);
    }

    #[test]
    fn summary() {
        let roll = Roll::new(2, 6, 3);
        assert_eq!(roll.get_summary(), "2d6+3");
    }

    #[test]
    fn from() {
        let roll = Roll::from("2d6+3");
        assert_eq!(roll.dices, vec![6, 6]);
        assert_eq!(roll.flat_bonus, 3);

        let roll = Roll::from("1d10+4");
        assert_eq!(roll.dices, vec![10]);
        assert_eq!(roll.flat_bonus, 4);
    }
}
