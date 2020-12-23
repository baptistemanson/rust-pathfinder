#[derive(Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug)]
pub enum Bonus {
    Enhancement,
    Natural,
    Alchemical,
}

#[derive(Debug)]
pub struct AbilityScore {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
    bonuses: Vec<BonusAbility>,
}

#[derive(Debug)]
pub struct BonusAbility {
    stat: Ability,
    bonus: Bonus,
    value: i32,
}

pub fn get_default_abilities() -> AbilityScore {
    AbilityScore {
        strength: 10,
        dexterity: 10,
        constitution: 10,
        intelligence: 10,
        wisdom: 10,
        charisma: 10,
        bonuses: vec![],
    }
}

pub fn get_default_bonus() -> BonusAbility {
    BonusAbility {
        bonus: Bonus::Enhancement,
        stat: Ability::Strength,
        value: 1,
    }
}
