use crate::utils::clamp;
use crate::FloatTy;
use crate::Vec3;

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
            r: clamp(v.x, 0.0, 1.0),
            g: clamp(v.y, 0.0, 1.0),
            b: clamp(v.z, 0.0, 1.0),
        }
    }

    pub fn gamma_corrected(self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn average(colors: &[Color]) -> Self {
        let len = colors.len() as FloatTy;

        let mut tr = 0.0;
        let mut tg = 0.0;
        let mut tb = 0.0;
        for color in colors {
            tr += color.r;
            tg += color.g;
            tb += color.b;
        }

        let r = tr / len;
        let g = tg / len;
        let b = tb / len;

        Color { r, g, b }
    }
}
