mod sprite_atlas;

use sprite_atlas::{Atlas, SpriteInWorld, SpriteVertex, SpriteWorld};

use crate::{camera::generate_cam_matrix, renderer_chain::Renderer, state::State};
use wgputils::{buffer::Buffer, Vertex};

use std::borrow::Cow;
use wgpu::{RenderPass, ShaderFlags};

use wgputils::{
    bind_group::BindGroupBuilder, cast_slice, pipeline::PipelineBuilder, sampler::Sampler,
};

pub struct SpriteRenderer {
    world_to_cam: wgpu::Buffer,
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
        bind_group_builder.set_resources(vec![&sampler, atlas, &world_to_cam]);

        let pipeline = PipelineBuilder::<SpriteVertex>::new(&device)
            .add_bind_group_layout(&bind_group_builder.get_layout())
            .set_vertex_shader(&module)
            .set_fragment_shader(&module)
            .build();

        // vertex
        let atlas = Atlas::new_from_grid([32., 32.], [32, 32]);
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
            world_to_cam: world_to_cam.buffer.unwrap(),
        }
    }
}
impl Renderer for SpriteRenderer {
    // Create command encoder
    // Create render pass
    // => Pick pipeline
    // => Pick bind group
    // => Pick index and vertex buffers
    // => Put Draw instruction in the render pass
    // Submit render pass to queue
    fn render<'a>(&'a mut self, mut rpass: RenderPass<'a>, queue: &wgpu::Queue, state: &State) {
        rpass.push_debug_group("Prepare data for draw.");
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        rpass.pop_debug_group();
        rpass.insert_debug_marker("Draw!");
        rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);

        let m = generate_cam_matrix(4. / 3., state.cam_pos);
        let m_ref: &[f32; 16] = m.as_ref();
        queue.write_buffer(&self.world_to_cam, 0, cast_slice(m_ref));
    }
}
