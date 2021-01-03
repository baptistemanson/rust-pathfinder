use rand::prelude::*;

// increasing dice size p279
#[allow(dead_code)]
pub fn increase_dice(x: i64) -> i64 {
    x + 2
}

pub fn dx(x: i64) -> i64 {
    thread_rng().gen_range(1..=x)
}
pub fn d20() -> i64 {
    thread_rng().gen_range(1..=20)
}
