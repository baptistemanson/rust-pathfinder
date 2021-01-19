use std::{collections::HashSet, time::Instant};

use crate::{
    buffer::Buffer,
    pipeline::PipelineBuilder,
    sampler::Sampler,
    texture::BatTex,
    utils::{self},
    vertex,
    world::mask_bit_tex,
};
use utils::cast_slice;
use vertex::Vertex;
use wgpu::util::DeviceExt;
use winit::event::{self, WindowEvent};

type KeyState = HashSet<event::VirtualKeyCode>;

pub struct TilesRenderer {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    scroll: wgpu::Buffer,
    curr_scroll: (f32, f32),
    last_update: Instant,
    key_state: KeyState,
}

impl crate::Renderer for TilesRenderer {
    // Describe each bind group layout
    // Assemble bind group layouts into a pipeline layout (aka bind groups[])
    // Describe the vertex layout
    // Load the shaders
    // Create the pipeline (aka the shaders + bind groups[] + vertex layout)
    //
    // Create the resources
    // Assemble resources in a bind group
    // Create vertex buffers
    // And... done!
    fn init(
        _sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let mut atlas = BatTex::image_tex(
            device,
            queue,
            include_bytes!("../assets/Tileset_32x32_1.png"),
            wgpu::ShaderStage::FRAGMENT,
        );
        let mut blueprint = mask_bit_tex(&device, &queue);
        let mut sampler = Sampler::new(&device);

        let mut blueprints_dim = Buffer::new(&device);
        blueprints_dim.init_data(cast_slice(&[20. as f32, 20. as f32]));

        let mut atlas_dim = Buffer::new(&device);
        atlas_dim.init_data(cast_slice(&[10. as f32, 10. as f32]));

        let mut output_dim = Buffer::new(&device);
        output_dim.init_data(cast_slice(&[12. as f32, 10. as f32]));

        let mut scroll = Buffer::new(&device);
        scroll.init_data(cast_slice(&[0. as f32, 0. as f32]));

        // Load shaders
        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("./shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("./shaders/shader.frag.spv"));

        let mut pipeline_builder = PipelineBuilder::<Vertex>::new(&device);
        pipeline_builder.add_to_bind_group(atlas_dim.get_layout());
        pipeline_builder.add_to_bind_group(atlas.get_layout());
        pipeline_builder.add_to_bind_group(sampler.get_layout());
        pipeline_builder.add_to_bind_group(blueprint.get_layout());
        pipeline_builder.add_to_bind_group(blueprints_dim.get_layout());
        pipeline_builder.add_to_bind_group(output_dim.get_layout());
        pipeline_builder.add_to_bind_group(scroll.get_layout());

        pipeline_builder.set_vertex_shader(vs_module);
        pipeline_builder.set_fragment_shader(fs_module);

        let (pipeline, bind_group_layout) = pipeline_builder.build();

        // Create a bind group, which is a collection of resources
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                atlas_dim.get_entry(0),
                atlas.get_entry(1),
                sampler.get_entry(2),
                blueprint.get_entry(3),
                blueprints_dim.get_entry(4),
                output_dim.get_entry(5),
                scroll.get_entry(6),
            ],
            label: None,
        });
        // Create the vertex and index buffers
        let (vertex_data, index_data) = vertex::quad();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&vertex_data), // checks if a range of bytes can be turned into another and just do it. Works well to turn Struct into u8
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(&index_data),
            usage: wgpu::BufferUsage::INDEX,
        });
        let index_count = index_data.len();

        TilesRenderer {
            pipeline,
            vertex_buf,
            index_buf,
            index_count,
            bind_group,
            scroll: scroll.buffer.unwrap(),
            curr_scroll: (0., 0.),
            last_update: Instant::now(),
            key_state: KeyState::default(),
        }
    }

    fn update(&mut self, event: &winit::event::WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    event::KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => match state {
                event::ElementState::Pressed => {
                    self.key_state.insert(key.clone());
                }
                event::ElementState::Released => {
                    if self.key_state.contains(&key) {
                        self.key_state.remove(&key);
                    }
                }
            },
            _ => {}
        }
    }

    fn resize(
        &mut self,
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    fn update_state(&mut self) {
        let delta = self.last_update.elapsed().as_secs_f32() * 5.;
        let mut delta_down = 0.;
        let mut delta_right = 0.;
        self.key_state.iter().for_each(|key| match key {
            event::VirtualKeyCode::Up => {
                delta_down -= delta;
            }
            event::VirtualKeyCode::Down => {
                delta_down += delta;
            }
            event::VirtualKeyCode::Left => {
                delta_right -= delta;
            }
            event::VirtualKeyCode::Right => {
                delta_right += delta;
            }
            _ => {}
        });
        let epsilon = 0.08;
        self.curr_scroll = (
            (self.curr_scroll.0 + delta_right)
                .max(0.)
                .min(20. - 12. - epsilon),
            (self.curr_scroll.1 + delta_down)
                .max(0.)
                .min(20. - 10. - epsilon),
        );
        self.last_update = Instant::now();
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
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
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
        queue.write_buffer(
            &self.scroll,
            0,
            cast_slice(&[self.curr_scroll.0, self.curr_scroll.1]),
        );
        queue.submit(Some(encoder.finish()));
    }
}
