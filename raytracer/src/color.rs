use crate::FloatTy;
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
            (clamp(v.x, 0.0, 1.0) * 255.0) as u8,
            (clamp(v.y, 0.0, 1.0) * 255.0) as u8,
            (clamp(v.z, 0.0, 1.0) * 255.0) as u8,
        )
    }

    pub fn gamma_corrected(self) -> Color {
        fn component(c: u8) -> u8 {
            let c = (c as f64) / 255.0;
            let c = c.sqrt();
            let c = c * 255.0;
            c as u8
        }

        Color::rgba(
            component(self.r),
            component(self.g),
            component(self.b),
            self.a,
        )
    }

    pub fn average(colors: &[Color]) -> Self {
        let mut tr: u64 = 0;
        let mut tg: u64 = 0;
        let mut tb: u64 = 0;
        let mut ta: u64 = 0;
        for color in colors {
            tr += color.r as u64;
            tg += color.g as u64;
            tb += color.b as u64;
            ta += color.a as u64;
        }

        let len = colors.len() as u64;
        let r = tr / len;
        let g = tg / len;
        let b = tb / len;
        let a = ta / len;
        Color::rgba(r as u8, g as u8, b as u8, a as u8)
    }
}

fn clamp(v: FloatTy, min: FloatTy, max: FloatTy) -> FloatTy {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}
