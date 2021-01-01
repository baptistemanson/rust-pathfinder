use crate::world::World;

pub fn find_first_target<'a>(party_id: &str, world: &'a World<'a>) -> Option<String> {
    let characters = world.get_characters();
    let ids: Vec<String> = characters
        .iter()
        .filter(|c| c.party != party_id)
        .map(|c| String::from(c.id))
        .collect();
    // only unit is current unit. Should not happen?
    if ids.len() == 0 {
        return None;
    }
    return Some(ids[0].clone());
}
