use wgputils::Vertex;

use crate::algebra::{make_box, Vec2};

#[repr(C)]
#[derive(Debug, Vertex, PartialEq)]
pub struct SpriteVertex {
    pos: [f32; 2],
    uv: [f32; 2],
    z: [f32; 1],
}

#[derive(Debug, PartialEq)]
pub struct Sprite {
    pub pos: Vec2, // in tiles dimension
    pub scale: f32,
    pub id_in_atlas: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Default for Sprite {
    fn default() -> Self {
        Sprite {
            pos: [0., 0.],
            scale: 1.,
            id_in_atlas: 0,
            flip_x: false,
            flip_y: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SpriteInAtlas {
    pub pos: Vec2,
    pub dim: Vec2,
}
#[derive(Debug, PartialEq)]
pub struct Atlas {
    pub sprite_by_id: Vec<SpriteInAtlas>,
    pub dim_units: Vec2, // in tiles / in meters.
}
#[derive(Debug, PartialEq)]
pub struct Sprites {
    pub dim_units: Vec2, // in tiles / in meters.
    pub sprites: Vec<Sprite>,
    pub atlas: Atlas,
}

impl Sprites {
    pub fn to_vertex(&self) -> Vec<SpriteVertex> {
        let mut acc: Vec<SpriteVertex> = vec![];

        for sprite in &self.sprites {
            let atlas_entry = &self.atlas.sprite_by_id[sprite.id_in_atlas as usize];
            let pos_box = make_box(sprite.pos, atlas_entry.dim);
            let z = [0.1]; //[pos_box.bottom_left[1]];

            let uv_box = make_box(atlas_entry.pos, atlas_entry.dim);
            let uv_box = uv_box.div(self.atlas.dim_units);

            let s_top_left = SpriteVertex {
                pos: pos_box.top_left,
                uv: uv_box.bottom_left,
                z,
            };
            let s_top_right = SpriteVertex {
                pos: pos_box.top_right,
                uv: uv_box.bottom_right,
                z,
            };
            let s_bottom_left = SpriteVertex {
                pos: pos_box.bottom_left,
                uv: uv_box.top_left,
                z,
            };
            let s_bottom_right = SpriteVertex {
                pos: pos_box.bottom_right,
                uv: uv_box.top_right,
                z,
            };
            acc.push(s_top_left);
            acc.push(s_top_right);
            acc.push(s_bottom_left);
            acc.push(s_bottom_right);
        }
        acc
    }

    pub fn to_indices(&self) -> Vec<u16> {
        let n = self.sprites.len();
        let mut out: Vec<u16> = Vec::with_capacity(6 * n);
        let n = n as u16;
        for i in 0..n {
            //  &[0, 1, 2, 1, 3, 2];
            out.push(4 * i);
            out.push(4 * i + 1);
            out.push(4 * i + 2);
            out.push(4 * i + 1);
            out.push(4 * i + 3);
            out.push(4 * i + 2);
        }
        out
    }

    pub fn to_index_count(&self) -> usize {
        self.sprites.len() * 6
    }
}

impl Atlas {
    // nb is the number of cells in the grid
    // dim is the size in units
    // dim == nb right now
    pub fn new_from_grid(dim: [f32; 2], nb: [u32; 2]) -> Self {
        let mut sprite_by_id: Vec<SpriteInAtlas> = vec![];
        let tile_width = dim[0] / nb[0] as f32;
        let tile_height = dim[1] / nb[1] as f32;
        for j in 0..nb[1] {
            for i in 0..nb[0] {
                let new_sprite = SpriteInAtlas {
                    pos: [i as f32 * tile_width, j as f32 * tile_height],
                    dim: [tile_width, tile_height],
                };
                sprite_by_id.push(new_sprite);
            }
        }
        Atlas {
            sprite_by_id,
            dim_units: dim,
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn atlas() {
        let atlas = Atlas::new_from_grid([10., 10.], [5, 5]);
        assert_eq!(atlas.sprite_by_id.len(), 25);
        assert_eq!(
            atlas.sprite_by_id[2],
            SpriteInAtlas {
                pos: [4., 0.],
                dim: [2., 2.]
            }
        )
    }
    #[test]
    fn to_vertex() {
        let atlas = Atlas::new_from_grid([2., 2.], [2, 2]);
        let sprites = vec![Sprite {
            id_in_atlas: 1,
            ..Sprite::default()
        }];
        let world = Sprites {
            atlas,
            dim_units: [100., 100.],
            sprites,
        };
        let vertices = world.to_vertex();
        assert_eq!(vertices[0].pos, [0., 0.]);
        // assert_eq!(vertices[0].uv, [0.5, 0.]);
        assert_eq!(vertices[3].pos, [0.01, 0.01]);
        //  assert_eq!(vertices[3].uv, [1.0, 0.5]);
    }
}
