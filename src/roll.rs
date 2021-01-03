use regex::Regex;
use std::ops;

type Dice = i64;

/// Represents a collection of dice and a flat bonus
/// Provides some convenient functionalities, like being to collect several number of dices from different bonuses,
/// or describe rolls with strings such as "1d6+1".
///
/// Can only model a dice roll with all dice of the same color (they can have different number of faces),
/// and cannot model complicated equations.
pub struct Roll {
    pub dices: Vec<Dice>,
    pub flat_bonus: i64,
}

impl Roll {
    /// Creates a new set of dice with a flat bonus.
    ///```
    ///use pathfinder::roll::Roll;
    ///assert_eq!(Roll::new(2, 6, 0).dices, vec![6,6]);
    ///```
    /// If you need different number of faces in the same roll, you can add rolls:
    ///```
    ///use pathfinder::roll::Roll;
    ///assert_eq!((Roll::new(1, 6, 0)+Roll::new(1, 4, 0)).dices, vec![6,4]);
    ///```
    pub fn new(nb_dice: usize, face: i64, flat_bonus: i64) -> Self {
        Roll {
            dices: vec![face; nb_dice],
            flat_bonus,
        }
    }

    ///```
    ///use pathfinder::roll::Roll;
    /// let roll = Roll::from("2d6+3");
    /// assert_eq!(roll.dices, vec![6, 6]);
    /// assert_eq!(roll.flat_bonus, 3);
    /// let roll = Roll::from("1d10+4");
    /// assert_eq!(roll.dices, vec![10]);
    /// assert_eq!(roll.flat_bonus, 4);
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
    /// Returns a human readable description of the current dice set.
    ///```
    ///use pathfinder::roll::Roll;
    ///assert_eq!(Roll::new(2, 6, 3).get_summary(), "2d6+3");
    ///assert_eq!(Roll::new(1, 10, 0).get_summary(), "1d10");
    ///assert_eq!(Roll::new(0, 0, 1).get_summary(), "+1");
    ///```
    pub fn get_summary(&self) -> String {
        match (self.dices.len(), self.flat_bonus) {
            (0, 0) => String::from(""),
            (0, x) => format!("+{}", x),
            (_, 0) => format!("{}d{}", self.dices.len(), self.dices[0]),
            _ => format!("{}d{}+{}", self.dices.len(), self.dices[0], self.flat_bonus),
        }
    }
    ///```
    ///use pathfinder::roll::Roll;
    ///let roll = Roll::new(6, 1, 3);
    ///assert_eq!(roll.resolve(), 9);
    ///```
    pub fn resolve(&self) -> i64 {
        self.dices.iter().fold(0, |a, b| a + b) + self.flat_bonus
    }
}

impl ops::Add<Roll> for Roll {
    type Output = Roll;

    ///```
    ///use pathfinder::roll::Roll;
    ///let roll = Roll::new(1, 6, 3) + Roll::new(2, 3, 5);
    ///assert_eq!(roll.dices, vec![6, 3, 3]);
    ///assert_eq!(roll.flat_bonus, 8);
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
        assert_eq!(roll.dices, vec![6]);
        assert_eq!(roll.flat_bonus, 3);
    }
    #[test]
    fn summary() {
        let roll = Roll::new(2, 6, 3);
        assert_eq!(roll.get_summary(), "2d6+3");
    }
}
