use rand::prelude::*;

pub fn d20() -> i64 {
    thread_rng().gen_range(1..=20) // up to 21 but not included
}
#[allow(dead_code)]
pub fn d8() -> i64 {
    thread_rng().gen_range(1..=8) // up to 21 but not included
}
#[allow(dead_code)]
pub fn d3() -> i64 {
    thread_rng().gen_range(1..=3) // up to 21 but not included
}
#[allow(dead_code)]
pub fn d4() -> i64 {
    thread_rng().gen_range(1..=4) // up to 21 but not included
}
#[allow(dead_code)]
pub fn d6() -> i64 {
    thread_rng().gen_range(1..=6) // up to 21 but not included
}
