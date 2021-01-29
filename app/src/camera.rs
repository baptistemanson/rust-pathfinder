pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
);

pub fn generate_cam_matrix(aspect_ratio: f32, pos: [f32; 3]) -> cgmath::Matrix4<f32> {
    let mx_projection = cgmath::perspective(cgmath::Deg(45f32), aspect_ratio, 0.5, 200.);
    let mx_view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(pos[0], pos[1], pos[2]),
        cgmath::Point3::new(pos[0], pos[1], 0.),
        cgmath::Vector3::unit_y(),
    );
    OPENGL_TO_WGPU_MATRIX * mx_projection * mx_view
}
