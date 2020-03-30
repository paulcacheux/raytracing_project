use crate::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_vec3(v: Vec3) -> Self {
        Color::rgb(
            (v.x * 255.0) as u8,
            (v.y * 255.0) as u8,
            (v.z * 255.0) as u8,
        )
    }
}
