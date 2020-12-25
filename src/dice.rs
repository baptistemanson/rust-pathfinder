use rand::prelude::*;

pub fn d20() -> u64 {
    thread_rng().gen_range(1..=20) // up to 21 but not included
}
