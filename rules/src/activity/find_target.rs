use crate::timeline::CharacterId;
use crate::world::World;
/**
Plenty of ideas:
- vicious: unconscious units
- assassin: the most DPS
- tank: on closest unit on line from self to the barycenter of enemy units (finding the front!)
- lazy: the closest
...
should read about it
*/
pub fn find_first_conscious_enemy(party: &str, world: &World) -> Option<CharacterId> {
    let characters = world.get_characters();
    let ids: Vec<String> = characters
        .iter()
        .filter(|c| c.party != party)
        .filter(|c| c.hp > 0)
        .map(|c| String::from(&c.id))
        .collect();
    // find unconscious
    if ids.len() == 0 {
        return find_first_enemy(party, world);
    }
    return Some(ids[0].clone());
}

pub fn find_first_enemy(party: &str, world: &World) -> Option<CharacterId> {
    let characters = world.get_characters();
    let ids: Vec<String> = characters
        .iter()
        .filter(|c| c.party != party)
        .map(|c| String::from(&c.id))
        .collect();
    // find unconscious
    if ids.len() == 0 {
        return None;
    }
    return Some(ids[0].clone());
}

pub fn find_all_friends(party: &str, world: &World) -> Vec<CharacterId> {
    world
        .get_characters()
        .iter()
        .filter_map(|c| {
            if c.party == party {
                Some(c.id.to_string())
            } else {
                None
            }
        })
        .collect()
}
