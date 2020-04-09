use std::sync::Arc;

use crate::fconsts;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::utils;
use crate::{FloatTy, Pt3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Pt3,
    pub normal: Vec3,
    pub uv_base: (Vec3, Vec3),
    pub material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: Pt3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Plane {
            point,
            normal,
            uv_base: (Vec3::zeros(), Vec3::zeros()),
            material,
        }
    }

    pub fn with_uv(
        point: Pt3,
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

    fn compute_uv(&self, p: Pt3) -> (FloatTy, FloatTy) {
        let (ub, vb) = self.uv_base;
        (ub.dot(&p.coords), vb.dot(&p.coords))
    }
}

impl Hittable for Plane {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let denominator = ray.direction.dot(&self.normal);
        if denominator.abs() <= fconsts::EPSILON {
            return None;
        }

        let t = (self.point - ray.origin).dot(&self.normal) / denominator;
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
