use crate::color::Color;

pub struct Material {
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
    pub emission: Color,
    pub reflection: Color,
}
