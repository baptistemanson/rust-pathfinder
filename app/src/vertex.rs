use bytemuck::{Pod, Zeroable};
// VERTEX BUFFER RELATED
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct VertexWithTex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}
#[allow(dead_code)]
fn vertex_with_tex(pos: [i8; 3], tc: [i8; 2]) -> VertexWithTex {
    VertexWithTex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}
#[allow(dead_code)]
pub fn cube() -> (Vec<VertexWithTex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex_with_tex([-1, -1, 1], [0, 0]),
        vertex_with_tex([1, -1, 1], [1, 0]),
        vertex_with_tex([1, 1, 1], [1, 1]),
        vertex_with_tex([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        vertex_with_tex([-1, 1, -1], [1, 0]),
        vertex_with_tex([1, 1, -1], [0, 0]),
        vertex_with_tex([1, -1, -1], [0, 1]),
        vertex_with_tex([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        vertex_with_tex([1, -1, -1], [0, 0]),
        vertex_with_tex([1, 1, -1], [1, 0]),
        vertex_with_tex([1, 1, 1], [1, 1]),
        vertex_with_tex([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        vertex_with_tex([-1, -1, 1], [1, 0]),
        vertex_with_tex([-1, 1, 1], [0, 0]),
        vertex_with_tex([-1, 1, -1], [0, 1]),
        vertex_with_tex([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        vertex_with_tex([1, 1, -1], [1, 0]),
        vertex_with_tex([-1, 1, -1], [0, 0]),
        vertex_with_tex([-1, 1, 1], [0, 1]),
        vertex_with_tex([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        vertex_with_tex([1, -1, 1], [0, 0]),
        vertex_with_tex([-1, -1, 1], [1, 0]),
        vertex_with_tex([-1, -1, -1], [1, 1]),
        vertex_with_tex([1, -1, -1], [0, 1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    _pos: [f32; 4],
}

fn vertex(pos1: f32, pos2: f32) -> Vertex {
    Vertex {
        _pos: [pos1, pos2, 0., 1.0],
    }
}

pub fn quad() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        vertex(-1., -1.),
        vertex(1., -1.),
        vertex(-1., 1.),
        vertex(1., 1.),
    ];

    let index_data: &[u16] = &[0, 1, 2, 1, 3, 2];

    (vertex_data.to_vec(), index_data.to_vec())
}
