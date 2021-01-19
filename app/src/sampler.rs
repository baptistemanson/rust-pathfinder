pub struct Sampler {}

impl Sampler {
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
}
