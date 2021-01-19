use crate::utils::get_pipeline_descriptor;
use crate::{utils::get_color_state, vertex_layout};
use wgpu::{BindGroupLayout, Device, RenderPipeline, ShaderModule};

pub struct PipelineBuilder<'a, Vertex> {
    device: &'a Device,
    pub bind_group_layout: Option<&'a BindGroupLayout>,
    pub vertex_description: Option<Vertex>,
    pub vertex_shader: Option<ShaderModule>,
    pub fragment_shader: Option<ShaderModule>,
}

impl<'a, Vertex> PipelineBuilder<'a, Vertex> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layout: None,
            vertex_description: None,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn set_bind_group_layout(
        &'a mut self,
        bind_group_layout: &'a BindGroupLayout,
    ) -> &'a mut Self {
        self.bind_group_layout = Some(bind_group_layout);
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

    pub fn build(&self) -> RenderPipeline {
        let vertex_state = vertex_layout![Vertex : 0 => Float4];

        let vs_module = &self
            .vertex_shader
            .as_ref()
            .unwrap_or_else(|| panic!("vertex shader is mandatory"));
        let fs_module = self
            .fragment_shader
            .as_ref()
            .unwrap_or_else(|| panic!("fragment shader is mandatory"));

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&self
                    .bind_group_layout
                    .as_ref()
                    .unwrap_or_else(|| panic!("bind group layout mandatory"))],
                push_constant_ranges: &[],
            });

        self.device.create_render_pipeline(&get_pipeline_descriptor(
            Some(&pipeline_layout),
            vertex_state,
            vs_module,
            fs_module,
            &[get_color_state(
                self.device.get_swap_chain_preferred_format(),
            )],
        ))
    }
}
