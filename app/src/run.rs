use std::borrow::Cow;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub async fn run(event_loop: EventLoop<()>, window: Window) {
    /*
    GENERAL SETUP CODE
    */
    // An instance is only a gateway to list and create adapters.
    // It also selects which backend renderer we will use, and wrap it in a context.
    // Here we could chanage the backend we use.
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    // Is the window we draw in. This is just a wrapper.
    // we could change the window we draw in.
    let surface = unsafe { instance.create_surface(&window) };

    // An adapter is choice of an API version + a physical 3d accelerator.
    // we could change for a more powerful API, or more conscious of power usage here.
    // LowPower will try to pick an integrated GPU first, then a discrete one.
    // HighPerformance will do the opposite.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    // the logical device is an instantiation of the adapter. It is in the owner and creator of all resources.
    // the command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    /*
    PREPARING THE PIPELINE
     */
    // SHADER LOAD
    // In WGSL, a single shader can desribe both fragment and vertex stages.
    // That way, descriptions for the varying can flow from the vertex to the fragment shader.
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None, // used for visual debuggers, to help identify which shader went wrong
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed("./shaders/shadre.vert")), // the sources themselves. Can be SpirV or WSGL.
        flags: wgpu::ShaderFlags::all(), // tells the level of validation that needs to happen. Mostly Naga related things that may disappear later.
    });

    // RESOURCES in WGPU (previously known as uniforms)
    // The pipeline layout describes the memory organization of the all data except vertex buffers.
    // Its goal is to establish a mapping between the resources and shader locations.
    //
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,               // idem
        bind_group_layouts: &[], // bind groups are a collection of uniforms. It allows to attach groups of uniforms in different pipelines.
        push_constant_ranges: &[], // push constants are tiny uniforms that are sent through the command buffer instead of a bind group.
    });

    // BIND GROUPS
    //
    // https://developer.nvidia.com/vulkan-shader-resource-binding#:~:text=Binding%20Resources%20as%20Groups%20with,using%20a%20different%20set%20number.
    // In essence, we can define groups of resources ahead of time and shared between pipelines.
    // For some reasons I don't know, binding something at runtime is costly. So we prefer to group things together and bind them all at once.
    //
    // Based on that, we usually group resources by update frequency (which optimize the number of bind calls required)
    // @todo figure out why 1 bind call is expensive.
    // NVIDIA seems to suggest the whole binding group is sent back to the GPU each time we bind (no change tracking?)

    // SWAP CHAIN TEXTURE FORMAT
    let swapchain_format = device.get_swap_chain_preferred_format();

    // MAIN CONSTRUCTION OF THE PIPELINE
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None, // idem
        layout: Some(&pipeline_layout),
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &shader,
            entry_point: "vs_main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &shader,
            entry_point: "fs_main",
        }),
        // Use the default rasterizer state: no culling, no depth bias
        rasterization_state: None,
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[swapchain_format.into()],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            // attributes and their organization in the buffer.
            index_format: None,
            vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    // SWAP CHAIN aka Output texture
    //
    let size = window.inner_size();
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    // Attach the output texture to the on screen window
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the swap chain with the new size
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::RedrawRequested(_) => {
                // grab the output frame
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;

                // queue of render passes.
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    // render pass is created from a descriptor
                    // describe the render target, the pipeline used....
                    //
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
