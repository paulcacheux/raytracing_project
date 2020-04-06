use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

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
    pub scattered: Ray,
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<MaterialScatter>;
    fn emit(&self, _u: FloatTy, _v: FloatTy, _point: Vec3) -> Vec3 {
        Vec3::all(0.0)
    }
}
