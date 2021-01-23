pub trait Vertex {
    fn get_descriptor() -> wgpu::VertexStateDescriptor<'static>;
    fn create_vertex_buffer(device: &Device, data: &[u8]) -> wgpu::Buffer;
    fn create_index_buffer(device: &Device, data: &[u8]) -> wgpu::Buffer;
}
pub use derive_vertex::Vertex;
use wgpu::Device;
