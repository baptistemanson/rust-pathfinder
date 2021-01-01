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
pub fn find_first_conscious_target<'a>(party_id: &str, world: &'a World<'a>) -> Option<String> {
    let characters = world.get_characters();
    let ids: Vec<String> = characters
        .iter()
        .filter(|c| c.party != party_id)
        .filter(|c| c.hp > 0)
        .map(|c| String::from(c.id))
        .collect();
    // should not happen?
    if ids.len() == 0 {
        return None;
    }
    return Some(ids[0].clone());
}
