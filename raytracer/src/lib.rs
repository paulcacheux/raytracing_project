mod camera;
mod color;
pub mod hittable;
pub mod material;
mod ray;
pub mod texture;
mod utils;
mod vec3;

pub type FloatTy = f64;

pub use crate::camera::*;
pub use crate::color::*;
pub use crate::hittable::{Hittable, HittableExt};
pub use crate::ray::*;
pub use crate::texture::Texture;
pub use crate::vec3::*;

pub fn compute_color(
    objects: &[Box<dyn Hittable>],
    ray: Ray,
    depth: usize,
    max_depth: usize,
    background: Vec3,
) -> Vec3 {
    if depth >= max_depth {
        return Vec3::all(0.0);
    }

    if let Some(record) = objects.is_hit_by(&ray, 0.01, None) {
        let emitted = record.material.emit(record.u, record.v, record.p);

        if let Some(material_scatter) = record.material.scatter(&ray, &record) {
            let scattered = material_scatter.scattered;
            let brdf = material_scatter.attenuation;

            /*
            let cos_theta = Vec3::dot(scattered.direction, record.normal);
            let scattered_color =
                compute_color(objects, scattered, depth + 1, max_depth, background);
            emitted + Vec3::memberwise_product(scattered_color, brdf) * cos_theta * 2.0
            */

            let scattered_color =
                compute_color(objects, scattered, depth + 1, max_depth, background);
            emitted + Vec3::memberwise_product(scattered_color, brdf)
        } else {
            emitted
        }
    } else {
        background
    }
}

pub mod fconsts {
    use super::FloatTy;
    pub const PI: FloatTy = std::f64::consts::PI as _;
    pub const EPSILON: FloatTy = std::f32::EPSILON as _;
    pub const FRAC_PI_2: FloatTy = std::f64::consts::FRAC_PI_2 as _;
}
