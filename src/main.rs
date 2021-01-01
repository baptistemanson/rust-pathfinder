// immutable
// try changing the API for something fully immutable? It would maybe be easier to deal with?
// would inform the ui on what needs redraw, but Ill probably redraw everything each frame...
// so I would have 1 state pointer?

// GENERAL
mod character;
mod dice;

// ITEMS and EQUIPMENT
mod item;
mod paladin;

// TIMELINE
mod activity;
mod timeline;

// UI
mod ui;
/**
* Findings:
* - I can have method/trait implementation across several files / several times in a file.
* - We can find some types with the same name, in different namespaces, bringing confusion (std::error::Error and std::io::Error)
* - I dont understand dyn, Box...
* - not fully understand how ::<>() works
* - structs are cool
* - I like the snake case convention, just weird that types are camelCase.
* - Return types are mandatory
* - build size is small (300KB with serde, 115KB gzipped), because it doesnt include all chrome/v8
* - ? wonder what would be the size with webgpu in it
* - build speed is good, compared to Typescript (3s release build, 1.80 dev build for a microproject). Seems to have 1.2s overhead... I wonder how people deal with larger project build time... relying more on the compiler?
* - the absence of "this" is dope.
* - I dont like the dep resolution by hand on serde. Sucks butt.
* - had a very weird linker issue when I tried to init a vec with the result of a function
* - string slice are pretty cool for immutable strings, but it requires some lifetime specs to allow rust to know when it can be dropped.
* - lifetimes are hard. If you borrow something, you probably want it to have its own lifetime, otherwise you could have transfered ownership.
*
* Derive
* - derive is asking the compiler to implement the trait for us. derive(Debug) is calling the macro Debug
* To implement a custom one: https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
* That would be ultimate
*
* - the absence of NULL and option everywhere is somewhat similar to the guarantee I have with Typescript at the end of the day.
*
* can do 100 inserts in 1ms in a hashmap? (should be doing 1,000 tbh)
* - so many data structures, its gonna take a while to pick the ones I like. I will start everything with the default vec, hashmaps and structs I think...


Cool stuff: was able to create a struct and a trait by myself.
*/
// flush trait/method is in std::io::Write
mod world;

use activity::select_best_action;
use timeline::{Activation, Timeline};
use world::World;

// Result is clear
// Box is because we cannot return a simple error straight, as errors can have different sizes
// so we put it in a box, and heap allocate the error.
// keyword dyn, I dont know why
type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_initiative<'a, 'b>(world: &'b World<'a>) -> Vec<Activation> {
    world
        .get_characters()
        .into_iter()
        .map(|c| Activation {
            character_id: String::from(c.id),
            initiative: c.roll_initiative(),
            party: String::from(c.party),
        })
        .collect()
}

// we split it out of the main function, as main here returns an int, and if we want to use ? we need to return Result.
fn resolve_encounter(mut world: &mut World) -> BoxResult<()> {
    {
        let mut timeline = Timeline::new();
        let mut is_encounter_done = false;

        // Step 1 - Initiative check p468
        let mut activations = get_initiative(&world);
        println!("Start of Round {}", timeline.turn_counter);
        ui::pause();
        while !is_encounter_done {
            // Step 2 - Play a round p468
            // @todo move that to select_best_activity?
            activations = activations
                .into_iter()
                .filter(|a| world.characters.get(&a.character_id).expect("oh no").hp > 0)
                .collect::<Vec<Activation>>();
            // println!("{:?}", activations);
            let tick = timeline.next_tick(&activations);
            match tick {
                timeline::Tick::Over => is_encounter_done = true,
                timeline::Tick::NewRound => {
                    ui::pause();
                    println!("Start of Round {}", timeline.turn_counter);
                }
                timeline::Tick::CharacterAction(c) => {
                    let active_character = world.get_character(&c).clone();
                    // had to clone because activity needs at the same time:
                    // an immutable ref to the character to know how much damage the attacker can do,
                    // a mutable ref to the world to resolve the action.

                    // @todo do not clone when I understand RefCell for performance reason.
                    let best_action = select_best_action(&active_character, world);
                    // collect effects of an activity as list of characters in the world
                    best_action.resolve(&active_character, &mut world);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let mut world = World::new();
    match resolve_encounter(&mut world) {
        Ok(()) => println!("Thank you for playing my game"),
        Err(e) => println!("Error: {}", e),
    }
}
