use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

use crate::{renderer_chain::Renderer, state::State};

use wgpu::RenderPass;

pub struct TextRenderer {
    glyph_brush: GlyphBrush<()>,
}

impl TextRenderer {
    pub fn init<'a>(device: &'a wgpu::Device, _state: &State) -> Self {
        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("../assets/Inconsolata-Regular.ttf"))
                .expect("Load font");

        let mut glyph_brush =
            GlyphBrushBuilder::using_font(font).build(device, wgpu::TextureFormat::Bgra8Unorm);

        TextRenderer { glyph_brush }
    }
}
impl Renderer for TextRenderer {
    // Create command encoder
    // Create render pass
    // => Pick pipeline
    // => Pick bind group
    // => Pick index and vertex buffers
    // => Put Draw instruction in the render pass
    // Submit render pass to queue
    fn render<'a>(&'a mut self, mut rpass: RenderPass<'a>, queue: &wgpu::Queue, _state: &State) {
        let section = Section {
            screen_position: (10.0, 10.0),
            text: vec![Text::new("Hello wgpu_glyph")],
            ..Section::default()
        };

        self.glyph_brush.queue(section);

        // self.glyph_brush
        //     .draw_queued(device, &mut encoder, &frame.view, frame.width, frame.height);

        //  device.get_queue().submit(&[encoder.finish()]);
    }
}
