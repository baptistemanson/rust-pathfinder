use crate::{
    character::Character,
    item::{
        armor::ArmorItem,
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

pub fn get_armor(character: &Character, world: &World) -> ArmorItem {
    let id = match &character.loadout.armor {
        Some(ref w) => w,
        None => "",
    };
    if id == "" {
        panic!("Didnt implement naked defense yet");
    } else {
        if let AnyItem::ArmorItem(item) = world.items.get(id).expect("cannot find weapon") {
            item.clone()
        } else {
            panic!("this is not a weapon")
        }
    }
}
