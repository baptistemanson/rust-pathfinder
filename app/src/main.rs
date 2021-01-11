mod framework;
mod utils;
mod vertex;

use wgpu::util::DeviceExt;

struct PathfinderApp {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_format: wgpu::IndexFormat,
    index_count: usize,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
}

impl PathfinderApp {}

// a sampler allows to sample the texture
// it takes a bit of time to instantiate, because it generates the mip maps...
fn sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    })
}
#[derive(Debug)]
struct BatTexDimensions {
    pub width: u32,
    pub height: u32,
}

// simple rgba texture.
#[derive(Debug)]
struct BatTex {
    pub bytes: Vec<u8>,
    pub dim: BatTexDimensions,
    format: wgpu::TextureFormat,
}
#[allow(dead_code)]
fn procedural_tex(size: u32) -> BatTex {
    BatTex {
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        dim: BatTexDimensions {
            width: size,
            height: size,
        },
        bytes: (0..size * size)
            .flat_map(|i| vec![(i % 256) as u8, 0, 0, 0])
            .collect::<Vec<u8>>(),
    }
}

fn pix(i: u8) -> Vec<u8> {
    vec![i, 0, 0, 0]
}
fn mask_bit_tex() -> BatTex {
    let bytes = vec![
        vec![34, 34, 34, 34, 34, 34, 34, 34],
        vec![34, 34, 34, 34, 34, 34, 34, 34],
        vec![17, 17, 17, 26, 17, 17, 17, 17],
        vec![17, 17, 17, 26, 17, 17, 17, 17],
        vec![17, 17, 17, 26, 17, 17, 17, 17],
        vec![17, 17, 17, 26, 17, 17, 17, 17],
        vec![17, 17, 17, 26, 17, 17, 17, 17],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let width = bytes[0].len();
    let height = bytes.len();
    BatTex {
        dim: BatTexDimensions {
            width: width as u32,
            height: height as u32,
        },
        format: wgpu::TextureFormat::Rgba8Unorm,
        bytes: bytes
            .into_iter()
            .flatten()
            .flat_map(|i| pix(i))
            .collect::<Vec<u8>>(),
    }
}

fn image_tex(path: &str) -> BatTex {
    let image = image::io::Reader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8();
    BatTex {
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        dim: BatTexDimensions {
            width: image.width(),
            height: image.height(),
        },
        bytes: image.into_raw(),
    }
}

// Grab a texture, send it to the queue, and returns the texture view.
fn texture(device: &wgpu::Device, queue: &wgpu::Queue, texture_bat: BatTex) -> wgpu::TextureView {
    let texture_extent = wgpu::Extent3d {
        width: texture_bat.dim.width,
        height: texture_bat.dim.height,
        depth: 1,
    };
    // the texture description.
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: texture_bat.format,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    let bytes_per_pixel = match texture_bat.format {
        wgpu::TextureFormat::R8Uint => 1,
        wgpu::TextureFormat::Rgba8UnormSrgb => 4,
        wgpu::TextureFormat::Rgba8Unorm => 4,
        _ => panic!("unknown format"),
    };
    // schedules the transfer of the texture data.
    queue.write_texture(
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &texture_bat.bytes,
        wgpu::TextureDataLayout {
            // weird to have to give the data layout again.
            // this defines a square subtexture
            offset: 0,
            bytes_per_row: bytes_per_pixel * texture_bat.dim.width,
            rows_per_image: 0,
        },
        texture_extent,
    );
    // texture view is used for the bind groups.
    // Texture views are used to specify which range of the texture is used by the shaders and how the data is interpreted.
    // allow for one texture to be shared between different shaders without having to change the shader.
    // the engine expects texture views in the binding group
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}

impl framework::App for PathfinderApp {
    fn optional_features() -> wgt::Features {
        wgt::Features::NON_FILL_POLYGON_MODE
    }

    fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        use std::mem;

        // Create the vertex and index buffers
        let vertex_size = mem::size_of::<vertex::Vertex>();
        let (vertex_data, index_data) = vertex::quad();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data), // checks if a range of bytes can be turned into another and just do it. Works well to turn Struct into u8
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsage::INDEX,
        });

        // Create pipeline layout, starting with the bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, //
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None, // 2 floats?
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
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create the texture

        let texture_tiles = texture(&device, &queue, image_tex("./assets/tiles.png"));
        let texture_mask = texture(&device, &queue, mask_bit_tex());

        let sampler = sampler(&device);

        // Buffer
        //convenient for everything but Samplers and Textures.
        let dim_tiles = [8. as f32, 6. as f32];
        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer transform"),
            contents: bytemuck::cast_slice(&dim_tiles), // [f32] => [u8]
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        // Bind groups!

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
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
            ],
            label: None,
        });

        let index_format = wgpu::IndexFormat::Uint16;

        // Vertices description
        let vertex_state = wgpu::VertexStateDescriptor {
            index_format: Some(index_format),
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float4,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float2,
                        offset: 4 * 4, // 4 float 32
                        shader_location: 1,
                    },
                ],
            }],
        };

        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: vertex_state.clone(),
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        // Done
        PathfinderApp {
            vertex_buf,
            index_buf,
            index_format,
            index_count: index_data.len(),
            bind_group,
            uniform_buf,
            pipeline,
        }
    }

    fn update(&mut self, _event: winit::event::WindowEvent) {
        //empty
    }

    fn resize(
        &mut self,
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    fn render(
        &mut self,
        frame: &wgpu::SwapChainTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _spawner: &framework::Spawner,
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
            rpass.set_index_buffer(self.index_buf.slice(..), self.index_format);
            rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
    }
}

fn main() {
    framework::run::<PathfinderApp>("pathfinder");
}
