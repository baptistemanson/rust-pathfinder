use crate::{
    camera::generate_cam_matrix,
    state::State,
    vertex::{self},
};
use std::borrow::Cow;
use vertex::VertexWithTex;
use wgpu::ShaderFlags;
use wgputils::cast_slice;

use wgputils::{
    bind_group::BindGroupBuilder, buffer::Buffer, pipeline::PipelineBuilder, sampler::Sampler,
};

pub struct TilesRenderer {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    world_to_cam: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl TilesRenderer {
    pub fn init<'a>(
        device: &'a wgpu::Device,
        _queue: &'a wgpu::Queue,
        atlas: &'a wgputils::texture::Texture<'a>,
        blueprint: &'a wgputils::texture::Texture<'a>,
        state: &State,
    ) -> Self {
        // Textures

        let sampler = Sampler::new(&device);

        let mut blueprints_dim = Buffer::new(&device, wgpu::ShaderStage::FRAGMENT);
        blueprints_dim.init_buffer(cast_slice(&[state.world_dim[0], state.world_dim[1]]));

        let mut atlas_dim = Buffer::new(&device, wgpu::ShaderStage::FRAGMENT);
        atlas_dim.init_buffer(cast_slice(&[32. as f32, 32. as f32]));

        let mut world_to_cam = Buffer::new(&device, wgpu::ShaderStage::VERTEX);
        let m = generate_cam_matrix(4. / 3., state.cam_pos);
        let m_ref: &[f32; 16] = m.as_ref();
        world_to_cam.init_buffer(cast_slice(m_ref));

        // Load shaders
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./tile/tile.wgsl"))),
            flags: ShaderFlags::VALIDATION,
        });

        let mut bind_group_builder = BindGroupBuilder::new(&device);
        bind_group_builder.set_resources(vec![
            &sampler,
            atlas,
            blueprint,
            &blueprints_dim,
            &atlas_dim,
            &world_to_cam,
        ]);

        let pipeline = PipelineBuilder::<VertexWithTex>::new(&device)
            .add_bind_group_layout(&bind_group_builder.get_layout())
            .set_vertex_shader(&module)
            .set_fragment_shader(&module)
            .build();

        // Create the vertex and index buffers
        let (vertex_buf, index_buf, index_count) =
            vertex::quad(&device, state.world_dim[0], state.world_dim[1]);

        TilesRenderer {
            pipeline,
            vertex_buf,
            index_buf,
            index_count,
            bind_group: bind_group_builder.get_bind_group(),
            world_to_cam: world_to_cam.buffer.unwrap(),
        }
    }
}
impl crate::Renderer for TilesRenderer {
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
        state: &State,
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

        let m = generate_cam_matrix(4. / 3., state.cam_pos);
        let m_ref: &[f32; 16] = m.as_ref();
        queue.write_buffer(&self.world_to_cam, 0, cast_slice(m_ref));

        queue.submit(Some(encoder.finish()));
    }
}
