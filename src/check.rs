use super::{character, dice};

pub fn get_modifier(score: i64) -> i64 {
    match score {
        i64::MIN..=1 => -5,
        _ => score / 2 - 5,
    }
}
impl<'a, 'b> character::Character<'a> {
    pub fn roll_perception_check(&self) -> i64 {
        // Perception check result = d20 roll + Wisdom modifier + proficiency bonus + other bonuses + penalties
        dice::d20() + get_modifier(self.ability_score.wisdom)
    }

    pub fn roll_initiative(&mut self) {
        self.initiative = self.roll_perception_check()
    }
}
