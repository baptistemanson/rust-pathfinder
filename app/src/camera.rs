pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
);

pub fn generate_matrix(aspect_ratio: f32) -> cgmath::Matrix4<f32> {
    let mx_projection = cgmath::perspective(cgmath::Deg(45f32), aspect_ratio, 1.0, 10.0);
    let mx_view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(1.5f32, -5.0, 3.0),
        cgmath::Point3::new(0f32, 0.0, 0.0),
        cgmath::Vector3::unit_z(),
    );
    OPENGL_TO_WGPU_MATRIX * mx_projection * mx_view
}

#[cfg(test)]
mod tests {
    use cgmath::Matrix4;

    use super::generate_matrix;

    #[test]
    fn get_matrix() {
        assert_eq!(
            generate_matrix(4. / 3.),
            Matrix4::new(
                1.7342978,
                -0.34566143,
                -0.27681828,
                -0.24913645,
                0.5202893,
                1.1522048,
                0.92272764,
                0.8304548,
                0.0,
                2.0931718,
                -0.55363655,
                -0.4982729,
                0.0,
                0.0,
                5.5786643,
                6.0207977
            )
        )
    }
}
