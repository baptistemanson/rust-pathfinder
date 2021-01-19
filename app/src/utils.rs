use wgpu::{ShaderModule, TextureFormat, VertexStateDescriptor};

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

// Get default pipeline descriptor
pub fn get_pipeline_descriptor<'a>(
    pipeline_layout: Option<&'a wgpu::PipelineLayout>,
    vertex_state: VertexStateDescriptor<'a>,
    vs_module: &'a ShaderModule,
    fs_module: &'a ShaderModule,
    color_states: &'a [wgpu::ColorStateDescriptor],
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        label: None,
        layout: pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            ..Default::default()
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states,
        depth_stencil_state: None,
        vertex_state,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    }
}

// TEXTURES
//
// a sampler allows to sample the texture
// it takes a bit of time to instantiate, because it generates the mip maps...
pub fn create_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    })
}
