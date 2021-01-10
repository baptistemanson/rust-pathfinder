use std::collections::HashMap;
/*
Timeline keeps track of rounds and which units already activated.
- unit and remove activations can be added on the fly, to simulate units joining and leaving the fight
- detects that a fight is over when there is no unit or only units of the same party

- does not support summon immediate activation the turn they are invoked.
- does not support yet informing the system fo the end of an effect.

/!\ Could be migrated to a generator if needed later I guess.


*/

// - A round is 6s. p13
#[allow(dead_code)]
fn get_min_in_rounds(min: i64) -> i64 {
    min * 10
}

#[derive(PartialEq, Debug)]
pub enum Tick {
    CharacterAction(CharacterId),
    NewRound,
    Over,
}
// TAG - CharacterId

pub type CharacterId = String;

pub type PartyId = String;

pub struct Timeline {
    pub turn_counter: i64,
    activated: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub struct Activation {
    pub character_id: CharacterId,
    pub party: PartyId,
    pub initiative: i64, // dont know if u64 is not better
}

fn _get_different_groups<'a>(activations: &'a Vec<Activation>) -> HashMap<String, u64> {
    let mut groups: HashMap<String, u64> = HashMap::new();
    for a in activations {
        let count = groups.entry(a.party.clone()).or_insert(0);
        *count += 1;
    }
    groups
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            turn_counter: 1,
            activated: Vec::new(),
        }
    }

    pub fn next_tick(&mut self, activations: &Vec<Activation>) -> Tick {
        // if there is only 1 party left, the fight is over
        let diff_active_groups = _get_different_groups(activations);
        if diff_active_groups.len() <= 1 {
            return Tick::Over;
        }

        // list of units to activate, per initiative
        let mut to_be_activated: Vec<&Activation> = activations
            .iter()
            .filter(|a| !self.activated.contains(&a.character_id))
            .collect::<Vec<&Activation>>();

        if to_be_activated.len() == 0 {
            self.activated = Vec::new();
            self.turn_counter += 1;
            return Tick::NewRound;
        }

        to_be_activated.sort_by(|a, b| a.initiative.cmp(&b.initiative));
        let next = to_be_activated[0].character_id.clone();
        self.activated.push(next.clone());
        return Tick::CharacterAction(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let timeline = Timeline::new();
        assert_eq!(timeline.turn_counter, 1);
    }
    #[test]
    fn encounter_nobody() {
        let mut timeline = Timeline::new();
        assert_eq!(timeline.next_tick(&vec![]), Tick::Over);
    }

    #[test]
    fn encounter_all_friendly() {
        let activations: Vec<Activation> = vec![
            Activation {
                character_id: String::from("a"),
                initiative: 0,
                party: String::from("1"),
            },
            Activation {
                character_id: String::from("b"),
                initiative: 0,
                party: String::from("1"),
            },
        ];
        let mut timeline = Timeline::new();
        assert_eq!(timeline.next_tick(&activations), Tick::Over);
    }

    #[test]
    fn encounter_2_units() {
        let activations: Vec<Activation> = vec![
            Activation {
                character_id: String::from("a"),
                initiative: 0,
                party: String::from("1"),
            },
            Activation {
                character_id: String::from("b"),
                initiative: 5,
                party: String::from("2"),
            },
        ];
        let mut timeline = Timeline::new();
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("a"))
        );
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("b"))
        );
        assert_eq!(timeline.next_tick(&activations), Tick::NewRound);
        assert_eq!(timeline.turn_counter, 2);
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("a"))
        );
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("b"))
        );
        assert_eq!(timeline.next_tick(&activations), Tick::NewRound);
    }

    #[test]
    fn encounter_unit_die_join() {
        let activations: Vec<Activation> = vec![
            Activation {
                character_id: String::from("a"),
                initiative: 0,
                party: String::from("1"),
            },
            Activation {
                character_id: String::from("b"),
                initiative: 5,
                party: String::from("2"),
            },
        ];
        let mut timeline = Timeline::new();
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("a"))
        );
        let activations = vec![
            Activation {
                character_id: String::from("b"),
                initiative: 0,
                party: String::from("2"),
            },
            Activation {
                character_id: String::from("c"),
                initiative: 10,
                party: String::from("1"),
            },
        ];
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("b"))
        );
        assert_eq!(
            timeline.next_tick(&activations),
            Tick::CharacterAction(String::from("c"))
        );
        assert_eq!(timeline.next_tick(&activations), Tick::NewRound);
    }
}

use dice::Roll;

use super::character;

pub fn get_modifier(score: i64) -> i64 {
    match score {
        i64::MIN..=1 => -5,
        _ => score / 2 - 5,
    }
}
impl character::Character {
    pub fn roll_perception_check(&self) -> i64 {
        // Perception check result = d20 roll + Wisdom modifier + proficiency bonus + other bonuses + penalties
        Roll::d("", 1, 20).roll() + get_modifier(self.ability_score.wisdom)
    }

    pub fn roll_initiative(&self) -> i64 {
        self.roll_perception_check()
    }
}
