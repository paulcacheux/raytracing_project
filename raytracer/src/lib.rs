use nalgebra::{Matrix4, Point3, Vector3};
use rand;
use rand::prelude::*;

mod camera;
mod color;
pub mod hittable;
pub mod material;
mod ray;
pub mod texture;
mod utils;

pub type FloatTy = f64;
pub type Vec3 = Vector3<FloatTy>;
pub type Pt3 = Point3<FloatTy>;
pub type Mat44 = Matrix4<FloatTy>;

pub use crate::camera::*;
pub use crate::color::*;
pub use crate::hittable::{Hittable, HittableExt};
pub use crate::ray::*;
pub use crate::texture::Texture;

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

                    scattered_color.component_mul(&brdf) / Q
                } else {
                    Vec3::zeros()
                }
            } else {
                Vec3::zeros()
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
