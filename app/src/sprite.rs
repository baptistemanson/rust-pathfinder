mod sprite_atlas;

use sprite_atlas::{Atlas, SpriteInWorld, SpriteVertex, SpriteWorld};

use crate::{camera::generate_cam_matrix, state::State};
use wgputils::{buffer::Buffer, Vertex};

use std::borrow::Cow;
use wgpu::ShaderFlags;

use wgputils::{
    bind_group::BindGroupBuilder, cast_slice, pipeline::PipelineBuilder, sampler::Sampler,
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
    pub fn init<'a>(
        device: &'a wgpu::Device,
        atlas: &'a wgputils::texture::Texture<'a>,
        state: &State,
    ) -> Self {
        // Textures

        let sampler = Sampler::new(&device);

        // Load shaders
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./sprite/sprite.wgsl"))),
            flags: ShaderFlags::VALIDATION,
        });

        let mut world_to_cam = Buffer::new(&device, wgpu::ShaderStage::VERTEX);
        let m = generate_cam_matrix(4. / 3., state.cam_pos);
        let m_ref: &[f32; 16] = m.as_ref();
        world_to_cam.init_buffer(cast_slice(m_ref));

        let mut bind_group_builder = BindGroupBuilder::new(&device);
        bind_group_builder.set_resources(vec![&sampler, atlas]);

        let pipeline = PipelineBuilder::<SpriteVertex>::new(&device)
            .add_bind_group_layout(&bind_group_builder.get_layout())
            .set_vertex_shader(&module)
            .set_fragment_shader(&module)
            .build();

        // vertex
        let atlas = Atlas::new_from_grid(state.world_dim, [32, 32]);
        let sprites = vec![
            SpriteInWorld {
                id_in_atlas: 40,
                ..SpriteInWorld::default()
            },
            SpriteInWorld {
                id_in_atlas: 168,
                pos: [3., 0.],
                ..SpriteInWorld::default()
            },
        ];

        let world = SpriteWorld {
            atlas: &atlas,
            dim_units: [6., 6.],
            sprites,
        };
        let vertex_buf =
            SpriteVertex::create_vertex_buffer(&device, cast_slice(&world.to_vertex()));
        let index_buf = SpriteVertex::create_index_buffer(&device, cast_slice(&world.to_indices()));
        let index_count = world.to_index_count();
        //  let (vertex_buf, index_buf, index_count) = quad(&device);
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
