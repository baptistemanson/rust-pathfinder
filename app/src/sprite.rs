use crate::vertex::{self, VertexPos};
use std::borrow::Cow;
use wgpu::ShaderFlags;
use wgputils::cast_slice;

use wgputils::{
    bind_group::BindGroupBuilder, buffer::Buffer, pipeline::PipelineBuilder, sampler::Sampler,
    texture::Texture,
};
use winit::event::WindowEvent;

pub struct SpriteRenderer {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl SpriteRenderer {
    fn init(
        _sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        // Textures
        let atlas = Texture::image_tex(
            device,
            queue,
            include_bytes!("../assets/Tileset_32x32_1.png"),
            wgpu::ShaderStage::FRAGMENT,
        );
        let sampler = Sampler::new(&device);

        let mut atlas_dim = Buffer::new(&device, wgpu::ShaderStage::FRAGMENT);
        atlas_dim.init_buffer(cast_slice(&[10. as f32, 10. as f32]));

        // Load shaders
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./shaders/sprite.wgsl"))),
            flags: ShaderFlags::VALIDATION,
        });

        let mut bind_group_builder = BindGroupBuilder::new(&device);
        bind_group_builder.set_resources(vec![&sampler, &atlas]);

        let pipeline = PipelineBuilder::<VertexPos>::new(&device)
            .add_bind_group_layout(&bind_group_builder.get_layout())
            .set_vertex_shader(&module)
            .set_fragment_shader(&module)
            .build();

        // Create the vertex and index buffers
        let (vertex_buf, index_buf, index_count) = vertex::quad(&device);

        SpriteRenderer {
            pipeline,
            vertex_buf,
            index_buf,
            index_count,
            bind_group: bind_group_builder.get_bind_group(),
        }
    }
}
impl crate::Renderer for SpriteRenderer {
    fn update(&mut self, _event: &WindowEvent) {}

    fn resize(
        &mut self,
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    fn update_state(&mut self) {}

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
