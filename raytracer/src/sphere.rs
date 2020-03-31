use std::sync::Arc;

use crate::intersectable::{Intersectable, IntersectionRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::Vec3;
use crate::FloatTy;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: FloatTy,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: FloatTy, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Intersectable for Sphere {
    fn is_intersected_by(
        &self,
        ray: &Ray,
        tmin: FloatTy,
        tmax: Option<FloatTy>,
    ) -> Option<IntersectionRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction) * 2.0;
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        if utils::is_in_range(t1, tmin, tmax) {
            let p = ray.point_at_parameter(t1);
            return Some(IntersectionRecord {
                t: t1,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material.clone(),
            });
        }

        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        if utils::is_in_range(t2, tmin, tmax) {
            let p = ray.point_at_parameter(t2);
            Some(IntersectionRecord {
                t: t2,
                p,
                normal: (p - self.center) / self.radius,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }
}
