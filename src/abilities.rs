#[derive(Debug, Clone)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone)]
pub enum Bonus {
    Enhancement,
    Natural,
    Alchemical,
}

#[derive(Debug, Clone)]
pub struct AbilityScore {
    pub strength: i64,
    pub dexterity: i64,
    pub constitution: i64,
    pub intelligence: i64,
    pub wisdom: i64,
    pub charisma: i64,
    pub bonuses: Vec<BonusAbility>,
}

#[derive(Debug, Clone)]
pub struct BonusAbility {
    pub stat: Ability,
    pub bonus: Bonus,
    pub value: i64,
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
