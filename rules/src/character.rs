use crate::{item::Loadout, status::StatusEffect};

use self::abilities::AbilityScore;
pub mod abilities;

#[derive(Clone, Default)]
pub struct Character {
    pub id: String,
    pub max_hp: i64,
    pub name: String,
    pub party: String,
    pub hp: i64,
    pub initiative: i64,
    pub ability_score: AbilityScore,
    pub loadout: Loadout,
    pub status: Vec<StatusEffect>,
}
impl Character {
    pub fn new(name: String, party: String, max_hp: i64) -> Character {
        Character {
            id: format!("{}-{}", name, snowflake::ProcessUniqueId::new()),
            name,
            party,
            max_hp,
            hp: max_hp,
            ..Default::default()
        }
    }

    // we can get negative HP in pathfinder
    #[allow(dead_code)]
    pub fn sub_hp(&mut self, hp: i64) {
        self.hp = self.hp.saturating_sub(hp);
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
        if self.hp <= 0 {
            println!("\t{} is unconscious!", self.name);
        }
    }
}
