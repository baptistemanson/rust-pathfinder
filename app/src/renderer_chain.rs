use wgpu::{Device, Queue, RenderPass};
use wgputils::texture::Texture;

use crate::{
    post_process_renderer::PostprocessRenderer,
    sprite_renderer::SpriteRenderer,
    state::State,
    tile_renderer::TilesRenderer,
    world::{lower, upper},
};

pub trait Renderer {
    fn render<'a>(&'a mut self, rpass: RenderPass<'a>, queue: &wgpu::Queue, state: &State);
}

pub struct RendererChain {
    pub renderers: Vec<Box<dyn Renderer>>,
    pub state: State,
}

impl RendererChain {
    pub fn new(device: &Device, queue: &Queue) -> Self {
        let tiles = Texture::image_tex(
            device,
            queue,
            include_bytes!("../assets/0x72_v1.3.png"),
            wgpu::ShaderStage::FRAGMENT,
        );

        let sprites = Texture::image_tex(
            device,
            queue,
            include_bytes!("../assets/0x72_v1.3.png"),
            wgpu::ShaderStage::FRAGMENT,
        );
        let lower = lower(device, queue);
        let upper = upper(device, queue);

        let my_world_state = State::my_world();
        let renderer1 = TilesRenderer::init(&device, &tiles, &lower, &my_world_state);
        let renderer2 = TilesRenderer::init(&device, &tiles, &upper, &my_world_state);
        let renderer3 = SpriteRenderer::init(&device, &sprites, &my_world_state);
        let renderer4 = PostprocessRenderer::init(&device, &my_world_state);

        Self {
            renderers: vec![
                Box::new(renderer1),
                Box::new(renderer2),
                Box::new(renderer3),
                Box::new(renderer4),
            ],
            state: my_world_state,
        }
    }

    pub fn render(
        &mut self,
        frame: &wgpu::SwapChainFrame,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        {
            self.state.update();

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.,
                            g: 0.,
                            b: 0.,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            self.renderers[0].render(rpass, &queue, &self.state);
            queue.submit(Some(encoder.finish()));
        }

        for renderer in self.renderers.iter_mut() {
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            renderer.render(rpass, &queue, &self.state);
            queue.submit(Some(encoder.finish()));
        }
    }
}
