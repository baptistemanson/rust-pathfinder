use wgputils::*;

make_answer!();

fn main() {
    println!("{}", answer());
}

pub struct Vertex {
    pub pos: [f32; 4],
    pub tex_coord: [f32; 2],
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(answer(), 42);
    }
    #[test]
    fn the_other_one() {
        println!("{:?}", get_vertex_layout!(Vertex));
        assert_eq!(true, false);
    }
}
