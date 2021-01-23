use crate::bindable::Bindable;

pub struct Sampler<'a> {
    _device: &'a wgpu::Device,
    sampler: wgpu::Sampler,
}

impl<'a> Bindable<'a> for Sampler<'a> {
    fn get_layout(&self, binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding, // will be remapped
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler {
                comparison: false,
                filtering: true,
            },
            count: None,
        }
    }

    fn get_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(&self.sampler),
        }
    }
}

impl<'a> Sampler<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Sampler {
            _device: device,
            sampler,
        }
    }
}
