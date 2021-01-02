// use super::Character;

pub struct ResolvedBonus {
    pub value: i64,
    pub details: String,
}

#[allow(dead_code)]
pub struct AvailableBonus {
    pub name: BonusType,
    pub roll_type: RollType,
    pub resolve: dyn Fn(i32) -> i32,
}
#[allow(dead_code)]
pub enum BonusType {
    Deadly,
    Striking,
    Bless
}
#[allow(dead_code)]
pub enum RollType {
    AttackRoll,
    DamageRollPreCrit,
    DamageRollPostCrit,
}

// pub fn get_bonuses(character: &Character) -> Vec<Bonus> {
//     character
//         .loadout
//         .as_vec()
//         .iter()
//         .flat_map(|i| i.get_bonuses())
//         .collect()
// }
