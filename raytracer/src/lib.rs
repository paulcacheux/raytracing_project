mod camera;
mod color;
mod intersectable;
mod material;
mod plane;
mod ray;
mod sphere;
mod utils;
mod vec3;

pub type FloatTy = f32;

pub use crate::camera::*;
pub use crate::color::*;
pub use crate::intersectable::*;
pub use crate::material::*;
pub use crate::plane::*;
pub use crate::ray::*;
pub use crate::sphere::*;
pub use crate::vec3::*;

pub fn compute_color(objects: &[Box<dyn Intersectable>], ray: Ray, depth: usize) -> Vec3 {
    if let Some(record) = objects.is_intersected_by(&ray, 0.01, None) {
        let emitted = record.material.emit(record.u, record.v, record.p);

        if depth < 10 {
            if let Some(material_scatter) = record.material.scatter(&ray, &record) {
                return if let Some(scattered) = material_scatter.scattered {
                    let cos_theta = Vec3::dot(scattered.direction, record.normal);
                    let brdf = material_scatter.attenuation / (std::f64::consts::PI as FloatTy);
                    let p = 1.0 / (2.0 * std::f64::consts::PI as FloatTy);
                    let scattered_color = compute_color(objects, scattered, depth + 1);
                    emitted + Vec3::memberwise_product(scattered_color, brdf) * cos_theta / p
                } else {
                    emitted
                };
            }
        }
        emitted
    } else {
        Vec3::all(0.1)
    }
}
