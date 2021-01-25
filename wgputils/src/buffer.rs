use crate::bindable::Bindable;
use wgpu::util::DeviceExt;
use wgpu::Device;

pub struct Buffer<'a> {
    device: &'a Device,
    pub buffer: Option<wgpu::Buffer>,
    visibility: wgpu::ShaderStage,
}

impl<'a> Bindable<'a> for Buffer<'a> {
    fn get_layout(&self, binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding, // will be remapped
            visibility: self.visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    fn get_entry(&'a self, binding: u32) -> wgpu::BindGroupEntry<'a> {
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

impl<'a> Buffer<'a> {
    pub fn new(device: &'a Device, visibility: wgpu::ShaderStage) -> Self {
        Buffer {
            device,
            buffer: None,
            visibility,
        }
    }

    pub fn init_buffer(&mut self, contents: &[u8]) {
        self.buffer = Some(
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents, // [f32] => [u8]
                    usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                }),
        )
    }
}
