use std::time::{Duration, Instant};
#[allow(dead_code)]
type UnitId = u16;

#[allow(dead_code)]
pub enum EaseFun {
    Linear,
    EaseIn,
    EaseOut,
}

#[allow(dead_code)]
pub enum AnimationType {
    Position { subject: UnitId, to: [f32; 2] },
    InfoOverhead { subject: UnitId, text: String },
    LogScroll { nb_of_lines: i32 },
}

#[allow(dead_code)]
pub struct Animation {
    start: Option<Instant>,
    duration: Duration,
    details: AnimationType,
    is_loop: bool,
    ease: EaseFun,
}
