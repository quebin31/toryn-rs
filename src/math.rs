use glium::uniforms::{AsUniformValue, UniformValue};
use glm::{Mat4, Vec4};

pub struct GMat4(pub Mat4);

impl AsUniformValue for GMat4 {
    fn as_uniform_value(&self) -> UniformValue {
        let cols = self.0.as_array();
        let cols = [
            cols[0].as_array().to_owned(),
            cols[1].as_array().to_owned(),
            cols[2].as_array().to_owned(),
            cols[3].as_array().to_owned(),
        ];

        UniformValue::Mat4(cols)
    }
}

pub fn ortho(left: f32, rigth: f32, bottom: f32, top: f32) -> Mat4 {
    Mat4::new(
        Vec4::new(2. / (rigth - left), 0., 0., 0.),
        Vec4::new(0., 2. / (top - bottom), 0., 0.),
        Vec4::new(0., 0., -1., 0.),
        Vec4::new(
            -(rigth + left) / (rigth - left),
            -(top + bottom) / (top - bottom),
            0.,
            1.,
        ),
    )
}
