use wgpu::util::DeviceExt;
use wgpu::Device;

pub struct Buffer<'a> {
    device: &'a Device,
    pub buffer: Option<wgpu::Buffer>,
}

impl<'a> Buffer<'a> {
    pub fn new(device: &'a Device) -> Self {
        Buffer {
            device,
            buffer: None,
        }
    }

    pub fn get_layout(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0, // will be remapped
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    pub fn set_data(&mut self, contents: &[u8]) {
        self.buffer = Some(
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Atlas Dimensions in number of tiles"),
                    contents, // [f32] => [u8]
                    usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                }),
        )
    }

    pub fn get_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding, // should be implicit via a collection
            resource: wgpu::BindingResource::Buffer {
                buffer: &self.buffer.as_ref().expect("buffer not set"),
                offset: 0,
                size: None,
            },
        }
    }
}
