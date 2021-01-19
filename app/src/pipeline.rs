use crate::{utils::get_color_state, vertex_layout};
use wgpu::{BindGroupLayout, Device, RenderPipeline, ShaderModule, VertexStateDescriptor};

pub struct PipelineBuilder<'a, Vertex> {
    device: &'a Device,
    bind_group_layout: Option<BindGroupLayout>,
    _vertex_description: Option<Vertex>,
    vertex_shader: Option<ShaderModule>,
    fragment_shader: Option<ShaderModule>,
}

impl<'a, Vertex> PipelineBuilder<'a, Vertex> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layout: None,
            _vertex_description: None,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn set_bind_group_layout(&mut self, layout: BindGroupLayout) -> &mut Self {
        self.bind_group_layout = Some(layout);
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

    pub fn build(&mut self) -> RenderPipeline {
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
                bind_group_layouts: &[self
                    .bind_group_layout
                    .as_ref()
                    .expect("You need to set a bind group before building")],
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

// Get default pipeline descriptor
pub fn get_pipeline_descriptor<'a>(
    pipeline_layout: Option<&'a wgpu::PipelineLayout>,
    vertex_state: VertexStateDescriptor<'a>,
    vs_module: &'a ShaderModule,
    fs_module: &'a ShaderModule,
    color_states: &'a [wgpu::ColorStateDescriptor],
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        label: None,
        layout: pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            ..Default::default()
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states,
        depth_stencil_state: None,
        vertex_state,
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    }
}
