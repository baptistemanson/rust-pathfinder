use regex::Regex;
use std::ops;

type Dice = i64;

#[cfg(test)]
pub fn dx(x: i64) -> i64 {
    match x {
        0 => 0,
        1 => 1,
        x => x / 2,
    }
}
#[cfg(not(test))]
use rand::prelude::*;
#[cfg(not(test))]
pub fn dx(x: i64) -> i64 {
    thread_rng().gen_range(1..=x)
}

/// Represents "1d6+1"
/// The dice are currently organized as a vector of dice, for the case where a bonus is a combination of different dices.
///
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Bonus {
    pub dices: Vec<Dice>,
    pub flat_bonus: i64,
}

impl Bonus {
    pub fn get_details(&self) -> String {
        match (self.dices.len(), self.flat_bonus) {
            (0, 0) => String::from(""),
            (0, x) => format!("{}", x),
            (_, 0) => format!("{}d{}", self.dices.len(), self.dices[0]),
            _ => format!("{}d{}+{}", self.dices.len(), self.dices[0], self.flat_bonus),
        }
    }

    pub fn roll(&self) -> i64 {
        self.dices.iter().fold(0, |a, d| a + dx(*d)) + self.flat_bonus
    }
}

/// Represents a collection of dice and a flat bonuses.
/// Typically keeping track of: "sword 1d6 + str 2 + deadly 2d4".
///
/// Can roll several batches of dice at once. It memoizes a particular total for a roll.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Roll {
    bonuses: Vec<(String, Bonus)>,
    pub value: i64,
    has_been_rolled: bool,
}

impl Roll {
    /// Creates a new roll.
    ///
    pub fn new(tag: &str, nb_dice: usize, face: i64, flat_bonus: i64) -> Self {
        let mut bonuses = vec![];
        bonuses.push((
            tag.to_string(),
            Bonus {
                dices: vec![face; nb_dice],
                flat_bonus,
            },
        ));
        Roll {
            bonuses,
            ..Default::default()
        }
    }

    /// Creates a new roll that is only a flat bonus.
    /// In isolation, it has no value, but it can be used in combination with others.
    pub fn flat(tag: &str, flat_bonus: i64) -> Self {
        Self::new(tag, 0, 0, flat_bonus)
    }

    /// Creates a new roll with only dices.
    pub fn d(tag: &str, nb_dice: usize, face: i64) -> Self {
        Self::new(tag, nb_dice, face, 0)
    }

    pub fn d20() -> Self {
        Roll::d("", 1, 20)
    }

    /// Parses a string slice and returns a roll.
    pub fn from(expr: &str) -> Self {
        let re = Regex::new(r"^(?P<nb_dice>\d+)d(?P<faces>\d+)\+?(?P<flat>\d+)?$").unwrap();
        let matches = re.captures(expr).unwrap();
        let mut nb_dice = 0;
        let mut faces = 0;
        let mut flat_bonus = 0;

        if let Some(nb_dice_match) = matches.name("nb_dice") {
            nb_dice = nb_dice_match.as_str().parse::<usize>().unwrap();
        }
        if let Some(faces_match) = matches.name("faces") {
            faces = faces_match.as_str().parse::<i64>().unwrap();
        }
        if let Some(flat_bonus_match) = matches.name("flat") {
            flat_bonus = flat_bonus_match.as_str().parse::<i64>().unwrap();
        }

        Roll::new("", nb_dice, faces, flat_bonus)
    }

    /// Rolls the dice, and memoizes the total for later use.
    pub fn resolve(&mut self) -> i64 {
        if self.has_been_rolled {
            return self.value;
        }
        self.has_been_rolled = true;
        self.value = self.roll();
        self.value
    }

    /// Rolls the dice, but don't memoize the total.
    pub fn roll(&self) -> i64 {
        self.bonuses
            .iter()
            .fold(0, |acc, (_, bonus)| acc + bonus.roll())
    }

    /// Generates a human readable string detailing the formula in place.
    /// The goal is to provide nice tooltips to explain a roll to the user.
    pub fn get_details(&self) -> String {
        self.bonuses
            .iter()
            .map(|(k, b)| {
                if k == "" {
                    b.get_details()
                } else {
                    if b.get_details() == "" {
                        String::from("")
                    } else {
                        format!("{} {}", k, b.get_details())
                    }
                }
            })
            .filter(|s| s != "")
            .collect::<Vec<String>>()
            .join(" + ")
    }

    pub fn get_bonus(&self, key: &str) -> Bonus {
        self.bonuses
            .iter()
            .find(|(k, _)| k == key)
            .expect("could not find desired bonus")
            .1
            .clone()
    }

    pub fn cancel_bonus(&self, key: &str) -> Self {
        let mut out = self.clone();
        out.bonuses = vec![];
        for (k, bonus) in &self.bonuses {
            if k != key {
                out.bonuses.push((k.clone(), bonus.clone()));
            }
        }
        out
    }
}

impl Default for Roll {
    fn default() -> Self {
        Roll::flat("", 0)
    }
}

impl ops::Add<Roll> for Roll {
    type Output = Roll;

    fn add(self, rhs: Roll) -> Roll {
        // @todo if same label on bonuses, merge the vecs instead.
        let mut bonuses = vec![];
        bonuses.extend(self.bonuses);
        bonuses.extend(rhs.bonuses);
        Roll {
            bonuses,
            has_been_rolled: false,
            value: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let roll = Roll::new("", 1, 6, 0);
        assert_eq!(roll.bonuses.len(), 1);

        let roll = Roll::d("dex modifier", 2, 4);
        assert_eq!(roll.bonuses.len(), 1);

        let roll = Roll::flat("str bonus", 10);
        assert_eq!(roll.bonuses.len(), 1);
    }

    #[test]
    fn resolve() {
        let mut roll1 = Roll::new("", 5, 1, 5);
        assert_eq!(roll1.resolve(), 10);
        let roll2 = Roll::new("", 1, 6, 0);
        assert_eq!((roll1 + roll2).resolve(), 13);
    }

    #[test]
    fn add() {
        let roll1 = Roll::d("dex modifier", 5, 1);
        let roll2 = Roll::flat("str bonus", 10);
        assert_eq!((roll1 + roll2).resolve(), 15);
    }

    #[test]
    fn get_details() {
        assert_eq!(
            (Roll::d("str", 1, 6) + Roll::new("dex", 2, 4, 1) + Roll::flat("", 4)).get_details(),
            "str 1d6 + dex 2d4+1 + 4"
        );
    }

    #[test]
    fn get_details_filter_empty() {
        assert_eq!(
            (Roll::d("", 0, 0)
                + Roll::d("str", 1, 6)
                + Roll::new("dex", 2, 4, 1)
                + Roll::flat("", 0))
            .get_details(),
            "str 1d6 + dex 2d4+1"
        );
    }

    #[test]
    fn pathfinder() {
        let dmg_roll = Roll::d("sword", 1, 6) + Roll::flat("str", 2) + Roll::d("deadly", 1, 6);
        assert_eq!(dmg_roll.get_details(), "sword 1d6 + str 2 + deadly 1d6");
    }

    #[test]
    fn from() {
        assert_eq!(Roll::from("1d6+1"), Roll::new("", 1, 6, 1));
        assert_eq!(Roll::from("1d6+1").roll(), 4); // when testing the dice rolls for half their value.
        assert_eq!(Roll::from("1d20"), Roll::new("", 1, 20, 0));
    }

    #[test]
    fn cancel() {
        let roll = Roll::from("1d6+1")
            + Roll::new("to_be_deleted", 1, 6, 1)
            + Roll::new("to_be_kept", 1, 6, 1);
        assert_eq!(
            roll.cancel_bonus("to_be_deleted"),
            Roll::from("1d6+1") + Roll::new("to_be_kept", 1, 6, 1)
        );
        // when testing the dice rolls for half their value.
    }
}
