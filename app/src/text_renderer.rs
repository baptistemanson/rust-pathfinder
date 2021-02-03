use wgpu_glyph::{ab_glyph, GlyphBrush, GlyphBrushBuilder, Section, Text};

use crate::{renderer_chain::Renderer, state::State};

use wgpu::util::StagingBelt;

pub struct TextRenderer {
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
}

impl TextRenderer {
    pub fn init<'a>(device: &'a wgpu::Device, _state: &State) -> Self {
        let font =
            ab_glyph::FontArc::try_from_slice(include_bytes!("../assets/Inconsolata-Regular.ttf"))
                .expect("Load font");

        let glyph_brush =
            GlyphBrushBuilder::using_font(font).build(device, wgpu::TextureFormat::Bgra8UnormSrgb);
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        TextRenderer {
            glyph_brush,
            staging_belt,
        }
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
    fn render_low<'a>(
        &'a mut self,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        frame: &'a wgpu::SwapChainTexture,
        state: &State,
    ) {
        let dim_float = (
            (state.window_dim[0] / 2) as f32,
            (state.window_dim[1] / 2) as f32,
        );
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let section = Section::default()
            .with_layout(
                wgpu_glyph::Layout::default_single_line()
                    .h_align(wgpu_glyph::HorizontalAlign::Center)
                    .v_align(wgpu_glyph::VerticalAlign::Center),
            )
            .add_text(
                Text::new("And now with text")
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(50.0),
            )
            .with_screen_position(dim_float);

        self.glyph_brush.queue(section);
        self.glyph_brush
            .draw_queued(
                device,
                &mut self.staging_belt,
                &mut encoder,
                &frame.view,
                state.window_dim[0],
                state.window_dim[1],
            )
            .expect("failing"); // dimensions to be found elsewhere
        self.staging_belt.finish();
        queue.submit(Some(encoder.finish()));
    }
}
