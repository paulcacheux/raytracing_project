use crate::{FloatTy, Vec3};

mod checker;
mod perlin;
mod solid;
pub use checker::*;
pub use perlin::*;
pub use solid::*;

pub trait Texture: Send + Sync + std::fmt::Debug {
    fn value(&self, u: FloatTy, v: FloatTy) -> Vec3;
}
