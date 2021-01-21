#[allow(dead_code)]
fn main() {}

pub struct Vertex {
    pub pos: [f32; 4],
    pub tex_coord: [f32; 2],
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_other_one() {
        println!("{:?}", derive_vertex::get_vertex_layout!(Vertex));
    }
}
