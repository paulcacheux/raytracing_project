use std::sync::Arc;

use crate::fconsts;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::Vec3;
use crate::FloatTy;

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub uv_base: (Vec3, Vec3),
    pub material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Plane {
            point,
            normal,
            uv_base: (Vec3::all(0.0), Vec3::all(0.0)),
            material,
        }
    }

    pub fn with_uv(
        point: Vec3,
        normal: Vec3,
        uv_base: (Vec3, Vec3),
        material: Arc<dyn Material>,
    ) -> Self {
        Plane {
            point,
            normal,
            uv_base,
            material,
        }
    }

    fn compute_uv(&self, p: Vec3) -> (FloatTy, FloatTy) {
        let (ub, vb) = self.uv_base;
        (Vec3::dot(ub, p), Vec3::dot(vb, p))
    }
}

impl Hittable for Plane {
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let denominator = Vec3::dot(ray.direction, self.normal);
        if denominator.abs() <= fconsts::EPSILON {
            return None;
        }

        let t = Vec3::dot(self.point - ray.origin, self.normal) / denominator;
        if utils::is_in_range(t, tmin, tmax) {
            let p = ray.point_at_parameter(t);
            let (u, v) = self.compute_uv(p);
            Some(HitRecord::new(
                ray,
                t,
                p,
                self.normal,
                u,
                v,
                self.material.clone(),
            ))
        } else {
            None
        }
    }
}
