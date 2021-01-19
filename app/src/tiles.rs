use std::{collections::HashSet, time::Instant};

use utils::cast_slice;
use vertex::Vertex;
use wgpu::util::DeviceExt;
use winit::event::{self, WindowEvent};

use crate::{
    pipeline::PipelineBuilder,
    utils::{self, create_sampler, create_texture},
    vertex,
    world::{image_tex, mask_bit_tex},
};

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
        // Describe bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4, //
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5, //
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6, //
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Load shaders
        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("./shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("./shaders/shader.frag.spv"));

        let mut pipeline_builder = PipelineBuilder::<Vertex>::new(&device);
        let pipeline = pipeline_builder
            .set_bind_group_layout(&bind_group_layout)
            .set_fragment_shader(fs_module)
            .set_vertex_shader(vs_module)
            .build();

        // Create resources
        let texture_tiles = create_texture(
            &device,
            &queue,
            image_tex(include_bytes!("../assets/Tileset_32x32_1.png")),
        );
        let texture_mask = create_texture(&device, &queue, mask_bit_tex());
        let sampler = create_sampler(&device);

        let atlas_dim = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Atlas Dimensions in number of tiles"),
            contents: cast_slice(&[10. as f32, 10. as f32]), // [f32] => [u8]
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let blueprints_dim = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Blueprint Dimensions in number of tiles"),
            contents: cast_slice(&[20. as f32, 20. as f32]), // [f32] => [u8]
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let output_dim = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Output Dimensions in number of tiles"),
            contents: cast_slice(&[12. as f32, 10. as f32]), // [f32] => [u8]
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let scroll = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scroll"),
            contents: cast_slice(&[0. as f32, 0. as f32]), // [f32] => [u8]
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        // Create a bind group, which is a collection of resources
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &atlas_dim,
                        offset: 0,
                        size: None,
                    },
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_tiles),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&texture_mask),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &blueprints_dim,
                        offset: 0,
                        size: None,
                    },
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &output_dim,
                        offset: 0,
                        size: None,
                    },
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &scroll,
                        offset: 0,
                        size: None,
                    },
                },
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
            scroll,
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
