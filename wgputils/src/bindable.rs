// @todo when done with implementation, add the trait to the resources.
pub trait Bindable<'a> {
    fn get_layout(&self, binding: u32) -> wgpu::BindGroupLayoutEntry;
    fn get_entry(&'a self, binding: u32) -> wgpu::BindGroupEntry<'a>;
}
