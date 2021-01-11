pub struct BoxesRenderer {}

impl BoxesRenderer {}

impl crate::Renderer for BoxesRenderer {
    fn optional_features() -> wgt::Features {
        wgt::Features::NON_FILL_POLYGON_MODE
    }

    fn init(
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) -> Self {
        BoxesRenderer {}
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
        _frame: &wgpu::SwapChainTexture,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _spawner: &crate::Spawner,
    ) {
    }
}
