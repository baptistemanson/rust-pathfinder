use crate::{
    renderer_chain::Renderer,
    state::State,
    vertex::{self},
};
use vertex::VertexWithTex;

use wgpu::RenderPass;
use wgputils::{bind_group::BindGroupBuilder, pipeline::PipelineBuilder};

pub struct PostprocessRenderer {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl PostprocessRenderer {
    pub fn init<'a>(device: &'a wgpu::Device, _state: &State) -> Self {
        // Textures

        // Load shaders
        // let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        //     label: None,
        //     source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
        //         "./postprocess/vignette.wgsl"
        //     ))),
        //     flags: ShaderFlags::VALIDATION,
        // });

        let module =
            device.create_shader_module(&wgpu::include_spirv!("./post_process/vignette.spv"));

        let mut bind_group_builder = BindGroupBuilder::new(&device);
        bind_group_builder.set_resources(vec![]);

        let pipeline = PipelineBuilder::<VertexWithTex>::new(&device)
            .add_bind_group_layout(&bind_group_builder.get_layout())
            .set_vertex_shader(&module)
            .set_fragment_shader(&module)
            .build();

        // Create the vertex and index buffers
        let (vertex_buf, index_buf, index_count) = vertex::quad(&device, 2., 2.);

        PostprocessRenderer {
            pipeline,
            vertex_buf,
            index_buf,
            index_count,
            bind_group: bind_group_builder.get_bind_group(),
        }
    }
}
impl Renderer for PostprocessRenderer {
    // Create command encoder
    // Create render pass
    // => Pick pipeline
    // => Pick bind group
    // => Pick index and vertex buffers
    // => Put Draw instruction in the render pass
    // Submit render pass to queue
    fn render<'a>(&'a mut self, mut rpass: RenderPass<'a>, _queue: &wgpu::Queue, _state: &State) {
        rpass.push_debug_group("Prepare data for draw.");
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        rpass.pop_debug_group();
        rpass.insert_debug_marker("Draw!");
        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
    }
}
