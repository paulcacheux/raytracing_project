use super::Texture;
use crate::{FloatTy, Vec3};

use noise::NoiseFn;
use noise::Perlin;

#[derive(Debug)]
pub struct PerlinTexture {
    inner: Perlin,
    freq: FloatTy,
}

impl PerlinTexture {
    pub fn new(freq: FloatTy) -> Self {
        PerlinTexture {
            inner: Perlin::new(),
            freq,
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, u: FloatTy, v: FloatTy) -> Vec3 {
        let coeff = self
            .inner
            .get([self.freq * u as FloatTy, self.freq * v as FloatTy]);
        let coeff = (coeff + 1.0) / 2.0;
        Vec3::repeat(1.0) * coeff
    }
}
