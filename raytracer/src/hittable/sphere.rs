use std::sync::Arc;

use super::{HitRecord, Hittable, AABB};
use crate::fconsts;
use crate::material::Material;
use crate::utils;
use crate::{FloatTy, Ray, Vec3};

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

    fn compute_uv(&self, p: Vec3) -> (FloatTy, FloatTy) {
        let d = (p - self.center).to_unit();
        let phi = d.z.atan2(d.x);
        let theta = d.y.asin();
        let u = 1.0 - (phi + fconsts::PI) / (2.0 * fconsts::PI);
        let v = (theta + fconsts::FRAC_PI_2) / fconsts::PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
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
            let (u, v) = self.compute_uv(p);
            return Some(HitRecord::new(
                ray,
                t1,
                p,
                (p - self.center) / self.radius,
                u,
                v,
                self.material.clone(),
            ));
        }

        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        if utils::is_in_range(t2, tmin, tmax) {
            let p = ray.point_at_parameter(t2);
            let (u, v) = self.compute_uv(p);
            return Some(HitRecord::new(
                ray,
                t2,
                p,
                (p - self.center) / self.radius,
                u,
                v,
                self.material.clone(),
            ));
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.center - Vec3::all(self.radius),
            max: self.center + Vec3::all(self.radius),
        })
    }
}
