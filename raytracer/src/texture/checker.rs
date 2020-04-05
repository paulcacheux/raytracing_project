use super::Texture;
use crate::{FloatTy, Vec3};
use std::sync::Arc;

#[derive(Debug)]
pub struct CheckerTexture {
    pub coeff: FloatTy,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>, coeff: FloatTy) -> Self {
        CheckerTexture { even, odd, coeff }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: FloatTy, v: FloatTy) -> Vec3 {
        let coeff = (self.coeff * u).sin() * (self.coeff * v).sin();
        if coeff < 0.0 {
            self.odd.value(u, v)
        } else {
            self.even.value(u, v)
        }
    }
}
