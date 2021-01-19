use crate::utils::get_pipeline_descriptor;
use crate::{utils::get_color_state, vertex_layout};
use wgpu::{BindGroupLayout, BindGroupLayoutEntry, Device, RenderPipeline, ShaderModule};

pub struct PipelineBuilder<'a, Vertex> {
    device: &'a Device,
    bind_group_layout_entries: Vec<BindGroupLayoutEntry>,
    _vertex_description: Option<Vertex>,
    vertex_shader: Option<ShaderModule>,
    fragment_shader: Option<ShaderModule>,
}

impl<'a, Vertex> PipelineBuilder<'a, Vertex> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layout_entries: vec![],
            _vertex_description: None,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn add_to_bind_group(&mut self, mut entry: BindGroupLayoutEntry) -> &mut Self {
        // renumber bindings
        entry.binding = self.bind_group_layout_entries.len() as u32;
        self.bind_group_layout_entries.push(entry);
        self
    }

    pub fn set_vertex_shader(&mut self, vertex_shader: ShaderModule) -> &mut Self {
        self.vertex_shader = Some(vertex_shader);
        self
    }

    pub fn set_fragment_shader(&mut self, fragment_shader: ShaderModule) -> &mut Self {
        self.fragment_shader = Some(fragment_shader);
        self
    }

    pub fn build(self) -> (RenderPipeline, BindGroupLayout) {
        let vertex_state = vertex_layout![Vertex : 0 => Float4];

        let vs_module = &self
            .vertex_shader
            .as_ref()
            .unwrap_or_else(|| panic!("vertex shader is mandatory"));
        let fs_module = self
            .fragment_shader
            .as_ref()
            .unwrap_or_else(|| panic!("fragment shader is mandatory"));

        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &self.bind_group_layout_entries,
                });

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        (
            self.device.create_render_pipeline(&get_pipeline_descriptor(
                Some(&pipeline_layout),
                vertex_state,
                vs_module,
                fs_module,
                &[get_color_state(
                    self.device.get_swap_chain_preferred_format(),
                )],
            )),
            bind_group_layout,
        )
    }
}
