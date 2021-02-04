use crate::sprite_atlas::{Atlas, Sprite, Sprites};
use std::{collections::HashSet, time::Instant};
use winit::event::{self, WindowEvent};

type KeyState = HashSet<event::VirtualKeyCode>;

pub struct State {
    pub cam_pos: [f32; 3],
    pub world_dim: [f32; 2],
    pub window_dim: [u32; 2],
    pub sprite_pos: Sprites,
    pub key_down: KeyState,
    pub key_down_handled: KeyState,
    pub last_update: Instant,
    pub game_state: rules::GameState,
}

impl State {
    pub fn my_world() -> Self {
        let atlas = Atlas::new_from_grid([32., 32.], [32, 32]);
        let sprites = vec![
            Sprite {
                id_in_atlas: 40,
                ..Sprite::default()
            },
            Sprite {
                id_in_atlas: 168,
                pos: [3., 0.],
                ..Sprite::default()
            },
        ];

        let sprite_pos = Sprites {
            atlas,
            dim_units: [6., 6.],
            sprites,
        };

        State {
            sprite_pos,
            cam_pos: [0., 0., 20.],
            world_dim: [20., 20.],
            window_dim: [0, 0],
            last_update: Instant::now(),
            key_down: HashSet::default(),
            key_down_handled: HashSet::default(),
            game_state: rules::GameState::new(),
        }
    }
    // if key press is less than a frame...? BUG
    pub fn process_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    event::KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => match state {
                event::ElementState::Pressed => {
                    self.key_down.insert(key.clone());
                }
                event::ElementState::Released => {
                    if self.key_down.contains(&key) {
                        self.key_down.remove(&key);
                        if self.key_down_handled.contains(&key) {
                            self.key_down_handled.remove(&key);
                        }
                    }
                }
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let delta = self.last_update.elapsed().as_secs_f32() * 5.;
        let mut delta_down = 0.;
        let mut delta_right = 0.;
        let mut new_handled_keys: Vec<event::VirtualKeyCode> = vec![];
        let mut should_move_sim_forward = false;

        self.key_down.iter().for_each(|key| {
            // tracking new key press.
            // I decided to hook my event on key down instead of key up for reactivity purposes.
            let mut is_new_key_press = false;
            if !self.key_down_handled.contains(&key) {
                new_handled_keys.push(key.clone());
                is_new_key_press = true;
            }
            match key {
                event::VirtualKeyCode::Up => {
                    delta_down += delta;
                }
                event::VirtualKeyCode::Down => {
                    delta_down -= delta;
                }
                event::VirtualKeyCode::Left => {
                    delta_right -= delta;
                }
                event::VirtualKeyCode::Right => {
                    delta_right += delta;
                }
                event::VirtualKeyCode::Space => {
                    if is_new_key_press {
                        dbg!("space pressed");
                        should_move_sim_forward = true;
                    }
                }
                _ => {}
            }
        });
        self.key_down_handled.extend(new_handled_keys);
        // update sprite position
        self.sprite_pos
            .sprites
            .iter_mut()
            .for_each(|s| s.pos[0] = s.pos[0] + 0.01);

        // update cam
        self.cam_pos[0] += delta_right;
        self.cam_pos[1] += delta_down;

        // update game state
        if should_move_sim_forward {
            let display = self.game_state.tick();
            println!("{}", display);
        }
        self.last_update = Instant::now();
    }
}
