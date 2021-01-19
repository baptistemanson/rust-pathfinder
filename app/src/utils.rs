use wgpu::TextureFormat;

pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::{mem::size_of, slice::from_raw_parts};

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

// @todo: try to derive the fields from the struct
#[macro_export]
macro_rules! vertex_layout {
    ($T:ty : $($loc:expr => $fmt:ident),* $(,)?) => {
        wgpu::VertexStateDescriptor {
            index_format: Some(wgpu::IndexFormat::Uint16),
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: std::mem::size_of::<$T>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![$($loc => $fmt ,)*],
            }],
        };
    };
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
