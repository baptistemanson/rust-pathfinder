// use super::Character;

pub struct ResolvedBonus {
    pub value: i64,
    pub details: String,
    pub roll: Roll,
}

pub type Roll = (i64, i64, i64); // XdY + Z

// pub fn get_bonuses(character: &Character) -> Vec<Bonus> {
//     character
//         .loadout
//         .as_vec()
//         .iter()
//         .flat_map(|i| i.get_bonuses())
//         .collect()
// }
