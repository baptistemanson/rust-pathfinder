use renderer_chain::RendererChain;
use std::future::Future;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use wgpu::{Features, Limits};
use winit::{
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod post_process_renderer;
mod renderer_chain;
mod sprite_renderer;
mod text_renderer;
mod tile_renderer;

// other
mod algebra;
mod animation;
mod camera;
mod sprite_atlas;
mod state;

mod vertex;
mod world;

fn main() {
    run("pathfinder");
}

// An App.

struct Setup {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn setup(title: &str) -> Setup {
    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title(title);
    #[cfg(windows_OFF)] // TODO
    {
        use winit::platform::windows::WindowBuilderExtWindows;
        builder = builder.with_no_redirection_bitmap(true);
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        console_log::init().expect("could not initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    log::info!("Initializing the surface...");

    let backend = wgpu::BackendBit::PRIMARY;
    let power_preference = wgpu::PowerPreference::default();
    let instance = wgpu::Instance::new(backend);
    let (size, surface) = unsafe {
        let size = window.inner_size();
        let surface = instance.create_surface(&window);
        (size, surface)
    };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("No suitable GPU adapters found on the system!");

    #[cfg(not(target_arch = "wasm32"))]
    {
        let adapter_info = adapter.get_info();
        println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
    }

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: Features::default(),
                limits: Limits::default(),
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .expect("Unable to find a suitable GPU adapter!");

    Setup {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }
}

fn start(
    Setup {
        window,
        event_loop,
        instance,
        size,
        surface,
        adapter,
        device,
        queue,
    }: Setup,
) {
    let spawner = Spawner::new();
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        format: adapter.get_swap_chain_preferred_format(&surface),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    log::info!("Initializing the renderer...");
    let mut renderer_chain = RendererChain::new(&device, &queue, [size.width, size.height]);
    #[cfg(not(target_arch = "wasm32"))]
    let mut last_update_inst = Instant::now();

    log::info!("Entering render loop...");
    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter); // force ownership by the closure
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            event::Event::MainEventsCleared => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Clamp to some max framerate to avoid busy-looping too much
                    // (we might be in wgpu::PresentMode::Mailbox, thus discarding superfluous frames)
                    //
                    // winit has window.current_monitor().video_modes() but that is a list of all full screen video modes.
                    // So without extra dependencies it's a bit tricky to get the max refresh rate we can run the window on.
                    // Therefore we just go with 60fps - sorry 120hz+ folks!
                    let target_frametime = Duration::from_secs_f64(1.0 / 60.0);
                    let time_since_last_frame = last_update_inst.elapsed();
                    if time_since_last_frame >= target_frametime {
                        window.request_redraw();
                        last_update_inst = Instant::now();
                    } else {
                        *control_flow = ControlFlow::WaitUntil(
                            Instant::now() + target_frametime - time_since_last_frame,
                        );
                    }

                    spawner.run_until_stalled();
                }

                #[cfg(target_arch = "wasm32")]
                window.request_redraw();
            }
            event::Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                log::info!("Resizing to {:?}", size);
                sc_desc.width = if size.width == 0 { 1 } else { size.width };
                sc_desc.height = if size.height == 0 { 1 } else { size.height };
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
                renderer_chain.state.window_dim = [size.width, size.height];
            }
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    renderer_chain.state.process_event(&event);
                }
            },
            event::Event::RedrawRequested(_) => {
                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(_) => {
                        swap_chain = device.create_swap_chain(&surface, &sc_desc);
                        swap_chain
                            .get_current_frame()
                            .expect("Failed to acquire next swap chain texture!")
                    }
                };
                renderer_chain.render(&frame, &device, &queue);
            }
            _ => {}
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Spawner<'a> {
    executor: async_executor::LocalExecutor<'a>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> Spawner<'a> {
    fn new() -> Self {
        Self {
            executor: async_executor::LocalExecutor::new(),
        }
    }

    #[allow(dead_code)]
    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'a) {
        self.executor.spawn(future).detach();
    }

    fn run_until_stalled(&self) {
        while self.executor.try_tick() {}
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Spawner {}

#[cfg(target_arch = "wasm32")]
impl Spawner {
    fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub fn spawn_local(&self, future: impl Future<Output = ()> + 'static) {
        wasm_bindgen_futures::spawn_local(future);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run(title: &str) {
    let setup = pollster::block_on(setup(title));
    start(setup);
}

#[cfg(target_arch = "wasm32")]
pub fn run(title: &str) {
    let title = title.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        let setup = setup::<E>(&title).await;
        start::<E>(setup);
    });
}
