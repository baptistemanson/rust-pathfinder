use wgpu::{BindGroupLayoutEntry, Device, Queue, TextureView};

#[derive(Debug)]
pub struct BatTexDimensions {
    pub width: u32,
    pub height: u32,
}

// simple rgba texture.
#[derive(Debug)]
pub struct Texture<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    view: Option<TextureView>,
    pub bytes: Vec<u8>,
    pub dim: BatTexDimensions,
    pub format: wgpu::TextureFormat,
    pub visibility: wgpu::ShaderStage,
}

impl<'a> Texture<'a> {
    pub fn from_code(
        device: &'a Device,
        queue: &'a Queue,
        bytes: Vec<u8>,
        dim: BatTexDimensions,
        visibility: wgpu::ShaderStage,
    ) -> Self {
        let t = Texture {
            queue,
            device,
            visibility,
            view: None,
            format: wgpu::TextureFormat::Rgba8Unorm, // weird... should check
            dim,
            bytes,
        };
        t.transfer()
    }
    #[allow(dead_code)]
    pub fn procedural_tex(
        device: &'a mut Device,
        queue: &'a mut Queue,
        size: u32,
        visibility: wgpu::ShaderStage,
    ) -> Texture<'a> {
        let t = Texture {
            queue,
            device,
            visibility,
            view: None,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            dim: BatTexDimensions {
                width: size,
                height: size,
            },
            bytes: (0..size * size)
                .flat_map(|i| vec![(i % 256) as u8, 0, 0, 0])
                .collect::<Vec<u8>>(),
        };
        t.transfer()
    }

    pub fn image_tex(
        device: &'a Device,
        queue: &'a Queue,
        data: &'a [u8],
        visibility: wgpu::ShaderStage,
    ) -> Texture<'a> {
        let image = image::load_from_memory(data).unwrap().into_rgba8();
        let t = Texture {
            device,
            queue,
            visibility,
            view: None,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            dim: BatTexDimensions {
                width: image.width(),
                height: image.height(),
            },
            bytes: image.into_raw(),
        };
        t.transfer()
    }

    pub fn get_layout(&self) -> BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0, // will be renumbered later
            visibility: self.visibility,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
            },
            count: None,
        }
    }
    pub fn get_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(self.view.as_ref().unwrap()),
        }
    }

    fn transfer(mut self) -> Self {
        let texture_extent = wgpu::Extent3d {
            width: self.dim.width,
            height: self.dim.height,
            depth: 1,
        };
        // the texture description.
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let bytes_per_pixel = match self.format {
            wgpu::TextureFormat::R8Uint => 1,
            wgpu::TextureFormat::Rgba8UnormSrgb => 4,
            wgpu::TextureFormat::Rgba8Unorm => 4,
            _ => panic!("unknown format"),
        };
        // schedules the transfer of the texture data.
        self.queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &self.bytes,
            wgpu::TextureDataLayout {
                // weird to have to give the data layout again.
                // this defines a square subtexture
                offset: 0,
                bytes_per_row: bytes_per_pixel * self.dim.width,
                rows_per_image: 0,
            },
            texture_extent,
        );
        // texture view is used for the bind groups.
        // Texture views are used to specify which range of the texture is used by the shaders and how the data is interpreted.
        // allow for one texture to be shared between different shaders without having to change the shader.
        // the engine expects texture views in the binding group
        self.view = Some(texture.create_view(&wgpu::TextureViewDescriptor::default()));
        self
    }
}
