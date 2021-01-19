// @todo when done with implementation, add the trait to the resources.
trait Resource {
    fn get_layout() -> wgpu::BindGroupLayoutEntry;
    fn get_entry() -> wgpu::BindGroupEntry;
}
