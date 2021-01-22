use wgputils::Vertex;

#[derive(Vertex)]
pub struct VertexWithUV {
    pub position: [f32; 4],
    pub uv: [f32; 2],
    pub fun: [f32; 2],
}

#[allow(dead_code)]
fn main() {
    // let t = VertexWithUV {
    //     position: [1., 1., 1., 1.],
    //     uv: [1., 1.],
    //     fun: [0., 0.],
    // };
    dbg!(VertexWithUV::get_descriptor());
}
