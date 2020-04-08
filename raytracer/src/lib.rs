use rand;
use rand::prelude::*;

mod camera;
mod color;
pub mod hittable;
mod mat44;
pub mod material;
mod ray;
pub mod texture;
mod utils;
mod vec3;

pub type FloatTy = f64;

pub use crate::camera::*;
pub use crate::color::*;
pub use crate::hittable::{Hittable, HittableExt};
pub use crate::mat44::*;
pub use crate::ray::*;
pub use crate::texture::Texture;
pub use crate::vec3::*;

const Q: FloatTy = 0.7;

pub fn compute_color<R: Rng>(
    objects: &[Box<dyn Hittable>],
    ray: Ray,
    background: Vec3,
    rng: &mut R,
) -> Vec3 {
    if let Some(record) = objects.is_hit_by(ray, 0.01, None) {
        let emitted = record.material.emit(record.u, record.v, record.p);

        if let Some(material_scatter) = record.material.scatter(&record.ray, &record) {
            let scat_value = if rng.gen::<FloatTy>() < Q {
                if let Some(scattered) = material_scatter.scattered {
                    let brdf = material_scatter.attenuation;

                    /*
                    let cos_theta = Vec3::dot(scattered.direction, record.normal);
                    let scattered_color =
                        compute_color(objects, scattered, depth + 1, max_depth, background);
                    emitted + Vec3::memberwise_product(scattered_color, brdf) * cos_theta * 2.0
                    */

                    let scattered_color = compute_color(objects, scattered, background, rng);
                    Vec3::memberwise_product(scattered_color, brdf) / Q
                } else {
                    Vec3::zero()
                }
            } else {
                Vec3::zero()
            };

            emitted + scat_value
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
