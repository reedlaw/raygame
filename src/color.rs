use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ops::Add<Color> for Color {
    type Output = Color;
    
    fn add(self, _rhs: Color) -> Color {
        Color {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: (self.r as f32 * scalar) as u8,
            g: (self.g as f32 * scalar) as u8,
            b: (self.b as f32 * scalar) as u8,
        }
    }
}

impl Color {
    pub fn new(r1: f32, g1: f32, b1: f32) -> Color {
        let r = (r1 * 255.0) as u8;
        let g = (g1 * 255.0) as u8;
        let b = (b1 * 255.0) as u8;
        return Color { r: r, g: g, b: b };
    }
}
