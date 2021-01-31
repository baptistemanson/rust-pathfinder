use std::borrow::Cow;

use crate::{
    state::State,
    vertex::{self},
};
use vertex::VertexWithTex;

use wgpu::ShaderFlags;
use wgputils::{bind_group::BindGroupBuilder, pipeline::PipelineBuilder};

pub struct PostprocessRenderer {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl PostprocessRenderer {
    pub fn init<'a>(device: &'a wgpu::Device, _queue: &'a wgpu::Queue, _state: &State) -> Self {
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
            device.create_shader_module(&wgpu::include_spirv!("./postprocess/vignette.spv"));

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
impl crate::Renderer for PostprocessRenderer {
    fn update(&mut self, _event: &winit::event::WindowEvent) {}

    fn resize(
        &mut self,
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    // Create command encoder
    // Create render pass
    // => Pick pipeline
    // => Pick bind group
    // => Pick index and vertex buffers
    // => Put Draw instruction in the render pass
    // Submit render pass to queue
    fn render(
        &mut self,
        frame: &wgpu::SwapChainTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _spawner: &crate::Spawner,
        ops: wgpu::Operations<wgpu::Color>,
        _state: &State,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops,
                }],
                depth_stencil_attachment: None,
            });
            rpass.push_debug_group("Prepare data for draw.");
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }
        queue.submit(Some(encoder.finish()));
    }
}
