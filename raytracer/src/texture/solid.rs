use super::Texture;
use crate::{FloatTy, Vec3};

#[derive(Debug)]
pub struct SolidTexture {
    pub color: Vec3,
}

impl SolidTexture {
    pub fn new(color: Vec3) -> Self {
        SolidTexture { color }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _: FloatTy, _: FloatTy) -> Vec3 {
        self.color
    }
}
