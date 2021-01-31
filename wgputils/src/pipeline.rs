use crate::get_color_state;
use crate::Vertex;
use wgpu::{BindGroupLayout, Device, MultisampleState, RenderPipeline, ShaderModule, VertexState};

pub struct PipelineBuilder<'a, T: Vertex> {
    device: &'a Device,
    bind_group_layouts: Vec<&'a BindGroupLayout>,
    _vertex_description: Option<T>,
    vertex_shader: Option<&'a ShaderModule>,
    fragment_shader: Option<&'a ShaderModule>,
}
// rename to RenderPipelineBuilder
// set_bind_group_layout => add_bind_group_layout
impl<'a, T: Vertex> PipelineBuilder<'a, T> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            device,
            bind_group_layouts: vec![],
            _vertex_description: None,
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn add_bind_group_layout(&mut self, layout: &'a BindGroupLayout) -> &mut Self {
        self.bind_group_layouts.push(layout);
        self
    }

    pub fn set_vertex_shader(&mut self, vertex_shader: &'a ShaderModule) -> &mut Self {
        self.vertex_shader = Some(vertex_shader);
        self
    }

    pub fn set_fragment_shader(&mut self, fragment_shader: &'a ShaderModule) -> &mut Self {
        self.fragment_shader = Some(fragment_shader);
        self
    }

    pub fn build(&mut self) -> RenderPipeline {
        let vs_module = &self
            .vertex_shader
            .unwrap_or_else(|| panic!("vertex shader is mandatory"));
        let vertex_state = T::get_descriptor(&vs_module);
        let fs_module = self
            .fragment_shader
            .unwrap_or_else(|| panic!("fragment shader is mandatory"));

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &self.bind_group_layouts[..],
                push_constant_ranges: &[],
            });

        self.device.create_render_pipeline(&get_pipeline_descriptor(
            Some(&pipeline_layout),
            vertex_state,
            fs_module,
            &[get_color_state(wgpu::TextureFormat::Bgra8UnormSrgb)], //@todo use the preferred_format function instead?
        ))
    }
}

// Get default pipeline descriptor
pub fn get_pipeline_descriptor<'a>(
    pipeline_layout: Option<&'a wgpu::PipelineLayout>,
    vertex_state: VertexState<'a>,
    fs_module: &'a ShaderModule,
    color_states: &'a [wgpu::ColorTargetState],
) -> wgpu::RenderPipelineDescriptor<'a> {
    wgpu::RenderPipelineDescriptor {
        label: None,
        layout: pipeline_layout,
        vertex: vertex_state,
        fragment: Some(wgpu::FragmentState {
            module: fs_module,
            entry_point: "main",
            targets: color_states,
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    }
}
