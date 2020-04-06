use std::sync::Arc;

use crate::{FloatTy, Vec3};

mod checker;
mod image;
mod perlin;
mod solid;
pub use self::image::*;
pub use checker::*;
pub use perlin::*;
pub use solid::*;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn value(&self, u: FloatTy, v: FloatTy) -> Vec3;
}

impl<T: Texture> Texture for Arc<T> {
    fn value(&self, u: FloatTy, v: FloatTy) -> Vec3 {
        self.as_ref().value(u, v)
    }
}
