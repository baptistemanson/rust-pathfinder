pub mod activity;
pub mod character;
pub mod item;
pub mod rules;
pub mod status;
pub mod timeline;
pub mod ui;
pub mod utils;
pub mod world;

use activity::select_best_action;
use timeline::{Activation, Timeline};
use ui::log;
use world::{init, World};

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn get_initiative<'b>(world: &'b World) -> Vec<Activation> {
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

pub fn main_loop(will_pause: bool) -> BoxResult<()> {
    let mut world = World::new();
    init(&mut world);
    resolve_encounter(will_pause, &mut world)
}

pub struct GameState {
    world: World,
    timeline: Timeline,
    activations: Vec<Activation>,
    is_encounter_done: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        init(&mut world);
        GameState {
            activations: get_initiative(&world),
            world,
            timeline: Timeline::new(),
            is_encounter_done: false,
        }
    }

    pub fn tick(&mut self) -> String {
        if self.is_encounter_done {
            return String::from("OVER");
        }

        let activations = self
            .activations
            .iter()
            .filter(|a| {
                self.world
                    .characters
                    .get(&a.character_id)
                    .expect("oh no")
                    .hp
                    > 0
            })
            .map(|a| a.clone())
            .collect::<Vec<Activation>>();

        let tick = self.timeline.next_tick(&activations);
        match tick {
            timeline::Tick::Over => {
                self.is_encounter_done = true;
                return String::from("OVER");
            }

            timeline::Tick::NewRound => {
                self.world.tick_down();
                return format!("Start of Round {}", self.timeline.turn_counter);
            }
            timeline::Tick::CharacterAction(c) => {
                let mut action_left = 3;
                while action_left > 0 {
                    let active_character = self.world.get_character(&c).clone();
                    // had to clone because activity needs at the same time:
                    // an immutable ref to the character to know how much damage the attacker can do,
                    // a mutable ref to the world to resolve the action.

                    // @todo do not clone when I understand RefCell for performance reason.
                    let mut best_action =
                        select_best_action(&active_character, action_left, &self.world);
                    action_left = action_left - best_action.get_cost();
                    // collect effects of an activity as list of characters in the world
                    best_action.resolve(&active_character, &mut self.world);
                }
                return format!("ACTION OVER");
            }
        }
    }
}

// to migrate

// we split it out of the main function, as main here returns an int, and if we want to use ? we need to return Result.
fn resolve_encounter(will_pause: bool, mut world: &mut World) -> BoxResult<()> {
    {
        let mut timeline = Timeline::new();
        let mut is_encounter_done = false;

        // Step 1 - Initiative check p468
        let mut activations = get_initiative(&world);
        log(&format!("Start of Round {}", timeline.turn_counter));
        if will_pause {
            ui::pause();
        }
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
                    if will_pause {
                        ui::pause();
                    }
                    log(&format!("Start of Round {}", timeline.turn_counter));
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
