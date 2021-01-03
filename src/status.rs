use std::cmp::max;

use crate::character::Character;

#[derive(Clone, Debug)]
pub struct StatusEffect {
    pub status_type: StatusType,
    pub duration: Duration,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum StatusType {
    // Poison,
    Bless,
    // Unconscious,
    // Dead,
}

// order is important, as we use this to do the partial order
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Duration {
    // EndOfAction,
    // EndOfActivation,
    // EndOfRound,
    // StartOfNextActivation,
    Round(i64),
}

impl<'a> Character<'a> {
    // @todo other durations should be implemented as well I guess.
    pub fn tick_down(&mut self) {
        let new_status = self
            .status
            .iter()
            .filter_map(|s| match s.duration {
                Duration::Round(1) => None,
                Duration::Round(x) => Some(StatusEffect {
                    duration: Duration::Round(x - 1),
                    status_type: s.status_type,
                }),
            })
            .collect();
        self.status = new_status;
    }
    pub fn add_status(&mut self, status: StatusEffect) {
        let mut was_found = false;

        for s in self.status.iter_mut() {
            if status.status_type == s.status_type {
                s.duration = max(status.duration, s.duration);
                was_found = true;
            }
        }
        if !was_found {
            self.status.push(status);
        }
    }

    pub fn has_status(&self, status_type: StatusType) -> bool {
        if let Some(_) = self.status.iter().find(|s| s.status_type == status_type) {
            true
        } else {
            false
        }
    }
}
