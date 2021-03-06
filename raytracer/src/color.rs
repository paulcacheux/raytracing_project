use crate::utils::clamp;
use crate::{FloatTy, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: FloatTy,
    pub g: FloatTy,
    pub b: FloatTy,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        let r = r as FloatTy / 255.0;
        let g = g as FloatTy / 255.0;
        let b = b as FloatTy / 255.0;
        Color { r, g, b }
    }

    pub fn from_vec3(v: Vec3) -> Self {
        Color {
            r: v.x,
            g: v.y,
            b: v.z,
        }
    }

    pub fn to_rgb(self) -> [u8; 3] {
        fn component(c: FloatTy) -> u8 {
            let c = clamp(c.sqrt(), 0.0, 1.0);
            (c * 255.0) as u8
        }

        [component(self.r), component(self.g), component(self.b)]
    }
}
