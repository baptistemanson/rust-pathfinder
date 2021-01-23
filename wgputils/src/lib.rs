pub trait Vertex {
    fn get_descriptor() -> wgpu::VertexStateDescriptor<'static>;
    fn create_vertex_buffer(device: &Device, data: &[u8]) -> wgpu::Buffer;
    fn create_index_buffer(device: &Device, data: &[u8]) -> wgpu::Buffer;
}
pub use derive_vertex::Vertex;
use wgpu::{Device, TextureFormat};
pub mod bind_group;
pub mod bindable;
pub mod buffer;
pub mod pipeline;
pub mod sampler;
pub mod texture;
pub mod utils;

pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::{mem::size_of, slice::from_raw_parts};

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

// Get default color state, ready to blend stuff
pub fn get_color_state(format: TextureFormat) -> wgpu::ColorStateDescriptor {
    wgpu::ColorStateDescriptor {
        format: format,
        color_blend: wgpu::BlendDescriptor {
            operation: wgpu::BlendOperation::Add,
            src_factor: wgpu::BlendFactor::SrcAlpha,
            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
        },
        alpha_blend: wgpu::BlendDescriptor::REPLACE,
        write_mask: wgpu::ColorWrite::ALL,
    }
}
