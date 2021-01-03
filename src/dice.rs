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
