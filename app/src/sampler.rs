pub struct Sampler<'a> {
    device: &'a wgpu::Device,
    sampler: Option<wgpu::Sampler>,
}

impl<'a> Sampler<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Sampler {
            device,
            sampler: None,
        }
    }
    pub fn get_layout(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0, // will be remapped
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler {
                comparison: false,
                filtering: true,
            },
            count: None,
        }
    }

    // a sampler allows to sample the texture
    // it takes a bit of time to instantiate, because it generates the mip maps...
    pub fn create_sampler(&mut self) {
        self.sampler = Some(self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        }))
    }

    pub fn get_entry(&mut self, binding: u32) -> wgpu::BindGroupEntry {
        self.create_sampler();
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(&self.sampler.as_ref().unwrap()),
        }
    }
}
