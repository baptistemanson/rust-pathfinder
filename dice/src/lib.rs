use regex::Regex;
use std::ops;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MY_REGEX: Regex =
        Regex::new(r"^(?:(?P<nb_dice>\d+)d(?P<faces>\d+))?(?:\+(?P<flat>\d+))?$").unwrap();
}

#[cfg(test)]
pub fn dx(x: i8) -> i8 {
    match x {
        0 => 0,
        1 => 1,
        x => x / 2,
    }
}

// #[cfg(not(test))]
// use rand::prelude::*;
// #[cfg(not(test))]
// pub fn dx(x: i8) -> i8 {
//     thread_rng().gen_range(1..=x)
// }

//uncomment this to benchmark
#[cfg(not(test))]
pub fn dx(x: i8) -> i8 {
    match x {
        0 => 0,
        1 => 1,
        x => x - 2,
    }
}

/// Used internally by [Roll](struct.Roll.html)
///
/// Represents "1d6+1".
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Bonus {
    pub nb_dice: i8,
    pub face: i8,
    pub flat_bonus: i64,
}

impl Bonus {
    pub fn roll(&self) -> i64 {
        (1..=self.nb_dice)
            .map(|_| dx(self.face) as i64)
            .sum::<i64>()
            + self.flat_bonus
    }
}

impl Bonus {
    fn to_string(&self) -> Option<String> {
        match (self.nb_dice, self.flat_bonus) {
            (0, 0) => None,
            (0, x) => Some(format!("{}", x)),
            (_, 0) => Some(format!("{}d{}", self.nb_dice, self.face)),
            _ => Some(format!(
                "{}d{}+{}",
                self.nb_dice, self.face, self.flat_bonus
            )),
        }
    }
}
/// Represents a collection of dice and a flat bonuses.
///
/// When you play a RPG, you may end up having to keep track of a variety of modifiers and bonuses to perform a dice roll.
/// In Pathfinder for instance,  "sword 1d6 + str 2 + deadly 2d4" is a legit dice roll.
///
/// While one could simply add integers to get the final results, being able to access the details of the computation is now an expected user experience.
/// This struct allows to keep track of a collection of additive [Bonus](struct.Bonus.html) like: "sword 1d6 + str 2 + deadly 2d4".
/// Each [Bonus](struct.Bonus.html) corresponds to either a flat bonus (+1), a dice bonus (2d6), or a combination of both (10d6+6).
///```
/// use dice::Roll;
/// assert_eq!(
///   (Roll::from("1d6+1").tag("weapon") + Roll::from("2d4").tag("dex bonus") + Roll::from("+4")).to_string(),
///    "weapon 1d6+1 + dex bonus 2d4 + 4"
///);
///Roll::from("1d6+1").roll();
///```
/// It also allows for using monad-like/chaining roll resolution, where each rule takes a roll and adds its modifiers in function of what has been already resolved.
///
/// In order to get this nice description, you can call to_string():

/// A roll can also be rolled. The value can be memoized, to allow carrying the result of one roll around the system.
/// Rolls always yield half the dice face rounded down in tests (a d3 will yield 1 for instance). It allows for cleaner unit tests with no need for a seed.
///
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Roll {
    bonuses: Vec<(String, Bonus)>,
    pub value: i64,
    has_been_rolled: bool,
}

impl Roll {
    /// Parses a string slice and returns a roll.
    ///```
    ///use dice::Roll;
    ///assert_eq!(Roll::from("1d6+2"), Roll::new("", 1, 6, 2));
    ///assert_eq!(Roll::from("1d6"), Roll::new("", 1, 6, 0));
    ///assert_eq!(Roll::from("+2").tag("my_tag"), Roll::new("my_tag", 0, 0, 2));
    ///```
    pub fn from(expr: &str) -> Self {
        let matches = MY_REGEX.captures(expr).unwrap();
        let mut nb_dice = 0;
        let mut faces = 0;
        let mut flat_bonus = 0;

        if let Some(nb_dice_match) = matches.name("nb_dice") {
            nb_dice = nb_dice_match.as_str().parse::<i64>().unwrap();
        }
        if let Some(faces_match) = matches.name("faces") {
            faces = faces_match.as_str().parse::<i64>().unwrap();
        }
        if let Some(flat_bonus_match) = matches.name("flat") {
            flat_bonus = flat_bonus_match.as_str().parse::<i64>().unwrap();
        }

        Roll::new("", nb_dice, faces, flat_bonus)
    }

    /// Tags the latest bonus of a roll.
    ///```
    ///use dice::Roll;
    ///assert_eq!(Roll::from("+1").tag("flat"), Roll::new("flat", 0, 0, 1));
    ///assert_eq!( (Roll::from("+1")       + Roll::from("+2")).tag("another tag"),
    ///             Roll::new("", 0, 0, 1) + Roll::new("another tag", 0, 0, 2)
    ///);
    ///```
    pub fn tag(&mut self, tag: &str) -> Self {
        let last = self.bonuses.len() - 1;
        self.bonuses[last].0 = String::from(tag);
        self.clone()
    }

    /// Creates a new roll.
    ///Usually the method `Roll::from()`, `Roll::d()` or `Roll::flat` are more convenient.
    /// The tag is used to remember the origin of a bonus or extra roll.
    ///```
    ///use dice::Roll;
    ///let roll = Roll::new("", 1, 6, 2);
    ///assert_eq!(roll, dice::Roll::from("1d6+2"));
    ///```
    pub fn new(tag: &str, nb_dice: i64, face: i64, flat_bonus: i64) -> Self {
        let mut bonuses = vec![];
        bonuses.push((
            tag.to_string(),
            Bonus {
                nb_dice: nb_dice as i8,
                face: face as i8,
                flat_bonus,
            },
        ));
        Roll {
            bonuses,
            ..Default::default()
        }
    }

    /// Alias for "[tag] +X"
    ///```
    ///use dice::Roll;
    ///assert_eq!( Roll::flat("my_tag", 1), Roll::from("+1").tag("my_tag"));
    ///```
    pub fn flat(tag: &str, flat_bonus: i64) -> Self {
        Self::new(tag, 0, 0, flat_bonus)
    }

    /// Alias for "[tag] XdY"
    ///```
    ///use dice::Roll;
    ///assert_eq!(Roll::d("my_tag", 1, 6), Roll::from("1d6").tag("my_tag"));
    ///```
    pub fn d(tag: &str, nb_dice: i64, face: i64) -> Self {
        Self::new(tag, nb_dice, face, 0)
    }

    /// Rolls the dice.
    ///
    /// Value is memoized, so rolling several times get the same result.
    /// It allows for propagating a result through the system with its details.
    ///```
    ///use dice::Roll;
    ///let mut roll = Roll::d("my_tag", 1, 6);
    ///assert_eq!(roll.resolve(), roll.resolve()); //
    ///```
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
        self.bonuses.iter().map(|(_, bonus)| bonus.roll()).sum()
    }

    /// Get the detail about a bonus.
    ///
    /// Useful when some rules interact with another.
    /// For instance, "Finesse" in Pathfinder allows to use the DEX modifier in lieu of the STR modifier.
    pub fn get_bonus(&self, key: &str) -> Bonus {
        self.bonuses
            .iter()
            .find(|(k, _)| k == key)
            .expect("could not find desired bonus")
            .1
            .clone()
    }

    /// Get the detail about a bonus.
    ///
    /// Useful when some rules interact with another.
    /// For instance, "Finesse" in Pathfinder allows to use the DEX modifier in lieu of the STR modifier.
    ///```
    /// let roll = dice::Roll::from("1d4").tag("weapon") + dice::Roll::from("+4").tag("str");
    /// assert_eq!(roll.remove_bonus("str"), dice::Roll::from("1d4").tag("weapon"));
    ///
    ///```
    pub fn remove_bonus(&self, key: &str) -> Self {
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

impl ToString for Roll {
    fn to_string(&self) -> String {
        self.bonuses
            .iter()
            .filter_map(|(k, b)| {
                if k == "" {
                    b.to_string()
                } else {
                    if b.to_string() == None {
                        None
                    } else {
                        Some(k.to_owned() + " " + &b.to_string().unwrap())
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" + ")
    }
}
impl ops::Add<Roll> for Roll {
    type Output = Roll;

    fn add(mut self, mut rhs: Roll) -> Roll {
        if rhs.has_been_rolled || self.has_been_rolled {
            panic!("You cannot modify dice rolls after having resolved them");
        }
        // @todo if same label on bonuses, merge the vecs instead.
        self.bonuses.append(&mut rhs.bonuses);
        self.has_been_rolled = false;
        self.value = 0;
        self
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
        let roll2 = roll1.clone();
        assert_eq!(roll1.resolve(), 10);
        let roll3 = Roll::new("", 1, 6, 0);
        assert_eq!((roll2 + roll3).resolve(), 13);
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
            (Roll::d("str", 1, 6) + Roll::new("dex", 2, 4, 1) + Roll::flat("", 4)).to_string(),
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
            .to_string(),
            "str 1d6 + dex 2d4+1"
        );
    }

    #[test]
    fn pathfinder() {
        let dmg_roll = Roll::d("sword", 1, 6) + Roll::flat("str", 2) + Roll::d("deadly", 1, 6);
        assert_eq!(dmg_roll.to_string(), "sword 1d6 + str 2 + deadly 1d6");
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
            roll.remove_bonus("to_be_deleted"),
            Roll::from("1d6+1") + Roll::new("to_be_kept", 1, 6, 1)
        );
        // when testing the dice rolls for half their value.
    }
}
