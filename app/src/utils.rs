#[allow(dead_code)]
pub fn generate_matrix(aspect_ratio: f32) -> cgmath::Matrix4<f32> {
    let mx_projection = cgmath::perspective(cgmath::Deg(45f32), aspect_ratio, 1.0, 10.0);
    let mx_view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(1.5f32, -5.0, 3.0),
        cgmath::Point3::new(0f32, 0.0, 0.0),
        cgmath::Vector3::unit_z(),
    );
    let mx_correction = OPENGL_TO_WGPU_MATRIX;
    mx_correction * mx_projection * mx_view
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::{mem::size_of, slice::from_raw_parts};

    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

// a sampler allows to sample the texture
// it takes a bit of time to instantiate, because it generates the mip maps...
pub fn sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
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
    format: wgpu::TextureFormat,
}
#[allow(dead_code)]
pub fn procedural_tex(size: u32) -> BatTex {
    BatTex {
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        dim: BatTexDimensions {
            width: size,
            height: size,
        },
        bytes: (0..size * size)
            .flat_map(|i| vec![(i % 256) as u8, 0, 0, 0])
            .collect::<Vec<u8>>(),
    }
}

pub fn pix(i: u8) -> Vec<u8> {
    vec![i, 0, 0, 0]
}

pub fn mask_bit_tex() -> BatTex {
    let bytes = vec![
        vec![34, 17, 34, 34],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1],
    ];
    let width = bytes[0].len();
    let height = bytes.len();
    BatTex {
        dim: BatTexDimensions {
            width: width as u32,
            height: height as u32,
        },
        format: wgpu::TextureFormat::Rgba8Unorm,
        bytes: bytes
            .into_iter()
            .flatten()
            .flat_map(|i| pix(i))
            .collect::<Vec<u8>>(),
    }
}

pub fn image_tex(data: &[u8]) -> BatTex {
    let image = image::load_from_memory(data).unwrap().into_rgba8();
    BatTex {
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        dim: BatTexDimensions {
            width: image.width(),
            height: image.height(),
        },
        bytes: image.into_raw(),
    }
}

// Grab a texture, send it to the queue, and returns the texture view.
pub fn texture(
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
