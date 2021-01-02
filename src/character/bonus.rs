use super::Character;

pub struct Bonus {
    bonus_type: BonusType,
    roll_type: RollType,
    value: i64,
}

pub enum BonusType {
    Circumstancial,
    Item,
    Status,
}

pub enum RollType {
    AttackRoll,
    DamageRoll,
}

pub fn get_bonuses(character: &Character) -> Vec<Bonus> {
    character
        .loadout
        .as_vec()
        .iter()
        .flat_map(|i| i.get_bonuses())
        .collect()
}
