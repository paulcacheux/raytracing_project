use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{FloatTy, Pt3, Vec3};

mod dielectric;
mod lambertian;
mod light;
mod metal;
mod utils;
pub use dielectric::*;
pub use lambertian::*;
pub use light::*;
pub use metal::*;

#[derive(Debug, Clone)]
pub struct MaterialScatter {
    pub attenuation: Vec3,
    pub scattered: Option<Ray>,
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<MaterialScatter>;
    fn emit(&self, _u: FloatTy, _v: FloatTy, _point: Pt3) -> Vec3 {
        Vec3::repeat(0.0)
    }
}
