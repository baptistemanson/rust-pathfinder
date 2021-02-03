use crate::sprite_atlas::{Atlas, Sprite, Sprites};
use std::{collections::HashSet, time::Instant};

use winit::event::{self, WindowEvent};

type KeyState = HashSet<event::VirtualKeyCode>;

pub struct State {
    pub cam_pos: [f32; 3],
    pub world_dim: [f32; 2],
    pub window_dim: [u32; 2],
    pub sprite_pos: Sprites,
    pub keys: KeyState,
    pub last_update: Instant,
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

        let world = Sprites {
            atlas,
            dim_units: [6., 6.],
            sprites,
        };

        State {
            sprite_pos: world,
            cam_pos: [0., 0., 20.],
            world_dim: [20., 20.],
            window_dim: [0, 0],
            last_update: Instant::now(),
            keys: HashSet::default(),
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
                    self.keys.insert(key.clone());
                }
                event::ElementState::Released => {
                    if self.keys.contains(&key) {
                        self.keys.remove(&key);
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
        self.keys.iter().for_each(|key| match key {
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
            _ => {}
        });
        self.sprite_pos
            .sprites
            .iter_mut()
            .for_each(|s| s.pos[0] = s.pos[0] + 0.01);
        self.cam_pos[0] += delta_right;
        self.cam_pos[1] += delta_down;
        self.last_update = Instant::now();
    }
}
