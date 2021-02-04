pub mod activity;
pub mod character;
pub mod fact;
pub mod item;
pub mod rules;
pub mod status;
pub mod timeline;
pub mod ui;
pub mod utils;
pub mod world;

use activity::select_best_action;
use fact::Facts;
use timeline::{Activation, Timeline};
use world::{init, World};

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

    pub fn tick(&mut self) -> Facts {
        let mut facts = Facts::new();
        if self.is_encounter_done {
            facts.info("OVER");
            return facts;
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
                facts.info("OVER");
                return facts;
            }

            timeline::Tick::NewRound => {
                self.world.tick_down();
                facts.info(&format!("Start of Round {}", self.timeline.turn_counter));
                return facts;
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
                    best_action.resolve(&active_character, &mut self.world, &mut facts);
                }
                facts.info("OVER");
                return facts;
            }
        }
    }
}
