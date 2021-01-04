use crate::{
    character::Character,
    item::{
        weapon::{unarmed, WeaponItem},
        AnyItem,
    },
    world::World,
};

pub fn get_active_weapon(character: &Character, world: &World) -> WeaponItem {
    let id = match (&character.loadout.left_hand, &character.loadout.right_hand) {
        (_, Some(ref w)) => w, // @todo check the rules on lefty/righty rules.
        (Some(ref w), None) => w,
        (None, None) => "",
    };
    if id == "" {
        unarmed()
    } else {
        if let AnyItem::WeaponItem(item) = world.items.get(id).expect("cannot find weapon") {
            item.clone()
        } else {
            panic!("this is not a weapon")
        }
    }
}
