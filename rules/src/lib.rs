mod activity;
mod character;
mod item;
mod status;
mod timeline;
mod ui;
mod utils;
mod world;

mod rules;

use activity::select_best_action;
use timeline::{Activation, Timeline};
use world::{init, World};

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_initiative<'b>(world: &'b World) -> Vec<Activation> {
    world
        .get_characters()
        .into_iter()
        .map(|c| Activation {
            character_id: String::from(&c.id),
            initiative: c.roll_initiative(),
            party: String::from(&c.party),
        })
        .collect()
}

pub fn main_loop() -> BoxResult<()> {
    let mut world = World::new();
    init(&mut world);
    resolve_encounter(&mut world)
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
            activations = activations
                .into_iter()
                .filter(|a| world.characters.get(&a.character_id).expect("oh no").hp > 0)
                .collect::<Vec<Activation>>();

            let tick = timeline.next_tick(&activations);
            match tick {
                timeline::Tick::Over => is_encounter_done = true,
                timeline::Tick::NewRound => {
                    ui::pause();
                    println!("Start of Round {}", timeline.turn_counter);
                    world.tick_down()
                }
                timeline::Tick::CharacterAction(c) => {
                    let mut action_left = 3;
                    while action_left > 0 {
                        let active_character = world.get_character(&c).clone();
                        // had to clone because activity needs at the same time:
                        // an immutable ref to the character to know how much damage the attacker can do,
                        // a mutable ref to the world to resolve the action.

                        // @todo do not clone when I understand RefCell for performance reason.
                        let mut best_action =
                            select_best_action(&active_character, action_left, world);
                        action_left = action_left - best_action.get_cost();
                        // collect effects of an activity as list of characters in the world
                        best_action.resolve(&active_character, &mut world);
                    }
                }
            }
        }
    }
    Ok(())
}
