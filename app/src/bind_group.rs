use wgpu::BindGroupLayout;

use crate::bindable::Bindable;

#[allow(dead_code)]
pub struct BindGroupBuilder<'a> {
    device: &'a wgpu::Device,
    resources: Vec<&'a dyn Bindable<'a>>,
    layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new(device: &'a wgpu::Device) -> Self {
        Self {
            device,
            resources: vec![],
            layout_entries: vec![],
        }
    }

    pub fn set_resources(&mut self, b: Vec<&'a dyn Bindable<'a>>) -> &mut Self {
        self.resources = b;
        self
    }

    #[allow(dead_code)]
    pub fn add_to_next_binding(&mut self, b: &'a dyn Bindable<'a>) -> &mut Self {
        self.resources.push(b);
        self
    }

    pub fn get_layout(&mut self) -> BindGroupLayout {
        if self.layout_entries.len() == 0 {
            self.layout_entries = self
                .resources
                .iter()
                .enumerate()
                .map(|(i, r)| r.get_layout(i as u32))
                .collect();
        }
        self.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &self.layout_entries,
            })
    }

    pub fn get_bind_group(&mut self) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = self
            .resources
            .iter()
            .enumerate()
            .map(|(i, r)| r.get_entry(i as u32))
            .collect();

        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.get_layout(),
            entries: &entries,
            label: None,
        })
    }
}
