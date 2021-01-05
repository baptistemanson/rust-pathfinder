use crate::{
    character::Character,
    item::{
        armor::ArmorItem,
        weapon::{WeaponItem},
        AnyItem,
    },
    world::World,
};

pub fn get_active_weapon<'a>(character: &Character, world: &'a World) -> &'a WeaponItem {
    let mut id = match (&character.loadout.left_hand, &character.loadout.right_hand) {
        (_, Some(ref w)) => w, // @todo check the rules on lefty/righty rules.
        (Some(ref w), None) => w,
        (None, None) => "",
    };
    if id == "" {
        id = "unarmed";
    } 
        if let AnyItem::WeaponItem(item) = world.items.get(id).expect("cannot find weapon") {
            &item
        } else {
            panic!("this is not a weapon")
        }
    
}

pub fn get_armor<'a>(character: &Character, world: &'a World) -> &'a ArmorItem {
    let id = match &character.loadout.armor {
        Some(ref w) => w,
        None => "",
    };
    if id == "" {
        panic!("Didnt implement naked defense yet");
    } else {
        if let AnyItem::ArmorItem(item) = world.items.get(id).expect("cannot find weapon") {
            &item
        } else {
            panic!("this is not a weapon")
        }
    }
}
