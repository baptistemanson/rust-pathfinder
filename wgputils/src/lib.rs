pub trait Vertex {
    fn get_descriptor() -> wgpu::VertexStateDescriptor<'static>;
}
pub use derive_vertex::Vertex;
