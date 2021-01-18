extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro]
pub fn get_vertex_layout(_item: TokenStream) -> TokenStream {
    "::wgpu::VertexStateDescriptor {
        index_format: Some(::wgpu::IndexFormat::Uint16),
        vertex_buffers: &[::wgpu::VertexBufferDescriptor {
            stride: 4 as ::wgpu::BufferAddress,
            step_mode: ::wgpu::InputStepMode::Vertex,
            attributes: &::wgpu::vertex_attr_array![0 => Float4],
        }],
    }"
    .parse()
    .unwrap()
}

// derive trait Vertex type
// take the struct, create a method getStride, and another one to getAttributes.
