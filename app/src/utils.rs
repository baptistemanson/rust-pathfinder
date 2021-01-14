pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::{mem::size_of, slice::from_raw_parts};

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

// I wished I could have writtent a function instead of this...
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

#[derive(Debug)]
pub struct BatTexDimensions {
    pub width: u32,
    pub height: u32,
}

// simple rgba texture.
#[derive(Debug)]
pub struct BatTex {
    pub bytes: Vec<u8>,
    pub dim: BatTexDimensions,
    pub format: wgpu::TextureFormat,
}

// Grab a texture, send it to the queue, and returns the texture view.
pub fn create_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_bat: BatTex,
) -> wgpu::TextureView {
    let texture_extent = wgpu::Extent3d {
        width: texture_bat.dim.width,
        height: texture_bat.dim.height,
        depth: 1,
    };
    // the texture description.
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture_bat.format,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    let bytes_per_pixel = match texture_bat.format {
        wgpu::TextureFormat::R8Uint => 1,
        wgpu::TextureFormat::Rgba8UnormSrgb => 4,
        wgpu::TextureFormat::Rgba8Unorm => 4,
        _ => panic!("unknown format"),
    };
    // schedules the transfer of the texture data.
    queue.write_texture(
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &texture_bat.bytes,
        wgpu::TextureDataLayout {
            // weird to have to give the data layout again.
            // this defines a square subtexture
            offset: 0,
            bytes_per_row: bytes_per_pixel * texture_bat.dim.width,
            rows_per_image: 0,
        },
        texture_extent,
    );
    // texture view is used for the bind groups.
    // Texture views are used to specify which range of the texture is used by the shaders and how the data is interpreted.
    // allow for one texture to be shared between different shaders without having to change the shader.
    // the engine expects texture views in the binding group
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
