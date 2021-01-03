use regex::Regex;
use std::ops;

type Dice = i64;

/// Represents a collection of dice and a flat bonus
/// Provides some convenient functionalities, like being to collect several number of dices from different bonuses,
/// or describe rolls with strings such as "1d6+1".
///
/// Can only model a dice roll with all dice of the same color (they can have different number of faces),
/// and cannot model complicated equations.
#[derive(Clone, Default)]
pub struct Roll {
    pub dices: Vec<Dice>,
    pub flat_bonus: i64,
    pub value: i64,
    pub has_been_rolled: bool,
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
            value: 0,
            has_been_rolled: false,
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
            ..Default::default()
        }
    }

    // @todo doesnt handle dice with different faces.
    /// Returns a human readable description of the current dice set.
    ///```
    ///use pathfinder::roll::Roll;
    ///assert_eq!(Roll::new(2, 6, 3).get_summary(), "2d6+3");
    ///assert_eq!(Roll::new(1, 10, 0).get_summary(), "1d10");
    ///assert_eq!(Roll::new(0, 0, 1).get_summary(), "+1");
    ///```
    /// When the dice has been rolled, the rolled value is appended, such as:
    /// ```
    ///use pathfinder::roll::Roll;
    ///let mut roll = Roll::new(2, 1, 1);
    ///roll.resolve();
    ///assert_eq!(roll.get_summary(), "2d1+1 [3]");
    ///
    ///let mut roll = Roll::new(0, 0, 0);
    ///roll.resolve();
    ///assert_eq!(roll.get_summary(), "");
    ///```
    pub fn get_summary(&self) -> String {
        let roll_desc = match (self.dices.len(), self.flat_bonus) {
            (0, 0) => String::from(""),
            (0, x) => format!("+{}", x),
            (_, 0) => format!("{}d{}", self.dices.len(), self.dices[0]),
            _ => format!("{}d{}+{}", self.dices.len(), self.dices[0], self.flat_bonus),
        };
        if self.has_been_rolled && self.value > 0 {
            format!("{}={}", roll_desc, self.value)
        } else {
            roll_desc
        }
    }
    /// Rolls the dice and memorize results. Useful when we need to refer to the dice result in several places.
    ///```
    ///use pathfinder::roll::Roll;
    ///let mut roll = Roll::new(6, 1, 3);
    ///assert_eq!(roll.resolve(), 9);
    ///```
    pub fn resolve(&mut self) -> i64 {
        if self.has_been_rolled {
            panic!("This roll already happened");
        }
        self.has_been_rolled = true;
        self.value = self.roll();
        self.value
    }

    pub fn roll(&self) -> i64 {
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
        Roll {
            dices,
            flat_bonus,
            has_been_rolled: false,
            value: 0,
        }
    }
}

impl ops::Add<i64> for Roll {
    type Output = Roll;
    ///```
    ///use pathfinder::roll::Roll;
    ///let roll = Roll::new(1, 6, 3) + 4;
    ///assert_eq!(roll.dices, vec![6]);
    ///assert_eq!(roll.flat_bonus, 7);
    fn add(self, rhs: i64) -> Roll {
        Roll {
            flat_bonus: self.flat_bonus + rhs,
            ..self
        }
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
