use crate::{
    camera::generate_matrix,
    vertex::{self, VertexPos},
};
use std::{borrow::Cow, collections::HashSet};
use vertex::VertexWithTex;
use wgpu::ShaderFlags;
use wgputils::cast_slice;

use wgputils::{
    bind_group::BindGroupBuilder, buffer::Buffer, pipeline::PipelineBuilder, sampler::Sampler,
};
use winit::event::{self};

// type KeyState = HashSet<event::VirtualKeyCode>;

pub struct TilesRenderer {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    // last_update: Instant,
    // key_state: KeyState,
}

const WORLD_WIDTH: f32 = 20.;
const WORLD_HEIGHT: f32 = 20.;

impl TilesRenderer {
    pub fn init<'a>(
        device: &'a wgpu::Device,
        _queue: &'a wgpu::Queue,
        atlas: &'a wgputils::texture::Texture<'a>,
        blueprint: &'a wgputils::texture::Texture<'a>,
    ) -> Self {
        // Textures

        let sampler = Sampler::new(&device);

        let mut blueprints_dim = Buffer::new(&device, wgpu::ShaderStage::FRAGMENT);
        blueprints_dim.init_buffer(cast_slice(&[WORLD_WIDTH, WORLD_HEIGHT]));

        let mut atlas_dim = Buffer::new(&device, wgpu::ShaderStage::FRAGMENT);
        atlas_dim.init_buffer(cast_slice(&[32. as f32, 32. as f32]));

        let m = generate_matrix(4. / 3.);
        let m_ref: &[f32; 16] = m.as_ref();
        let mut world_to_cam = Buffer::new(&device, wgpu::ShaderStage::VERTEX);
        world_to_cam.init_buffer(cast_slice(m_ref));

        // Load shaders
        let module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("./shaders/tile.wgsl"))),
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
        let (vertex_buf, index_buf, index_count) = vertex::quad(&device, WORLD_WIDTH, WORLD_HEIGHT);

        TilesRenderer {
            pipeline,
            vertex_buf,
            index_buf,
            index_count,
            bind_group: bind_group_builder.get_bind_group(),
            //     last_update: Instant::now(),
            //     key_state: KeyState::default(),
        }
    }
}
impl crate::Renderer for TilesRenderer {
    fn update(&mut self, _event: &winit::event::WindowEvent) {
        // match event {
        //     WindowEvent::KeyboardInput {
        //         input:
        //             event::KeyboardInput {
        //                 virtual_keycode: Some(key),
        //                 state,
        //                 ..
        //             },
        //         ..
        //     } => match state {
        //         event::ElementState::Pressed => {
        //             self.key_state.insert(key.clone());
        //         }
        //         event::ElementState::Released => {
        //             if self.key_state.contains(&key) {
        //                 self.key_state.remove(&key);
        //             }
        //         }
        //     },
        //     _ => {}
        // }
    }

    fn resize(
        &mut self,
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    fn update_state(&mut self) {
        // let delta = self.last_update.elapsed().as_secs_f32() * 5.;
        // let mut delta_down = 0.;
        // let mut delta_right = 0.;
        // self.key_state.iter().for_each(|key| match key {
        //     event::VirtualKeyCode::Up => {
        //         delta_down -= delta;
        //     }
        //     event::VirtualKeyCode::Down => {
        //         delta_down += delta;
        //     }
        //     event::VirtualKeyCode::Left => {
        //         delta_right -= delta;
        //     }
        //     event::VirtualKeyCode::Right => {
        //         delta_right += delta;
        //     }
        //     _ => {}
        // });
        // let epsilon = 0.08;
        // self.curr_scroll = (
        //     (self.curr_scroll.0 + delta_right)
        //         .max(0.)
        //         .min(20. - WIDTH - epsilon),
        //     (self.curr_scroll.1 + delta_down)
        //         .max(0.)
        //         .min(20. - HEIGHT - epsilon),
        // );
        // self.last_update = Instant::now();
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
        // queue.write_buffer(
        //     &self.scroll,
        //     0,
        //     cast_slice(&[self.curr_scroll.0, self.curr_scroll.1]),
        // );
        queue.submit(Some(encoder.finish()));
    }
}
