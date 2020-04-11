use std::sync::Arc;

use super::{HitRecord, Hittable, AABB};
use crate::fconsts;
use crate::material::Material;
use crate::utils;
use crate::{FloatTy, Pt3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: Pt3,
    pub v1: Pt3,
    pub v2: Pt3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(
        v0: Pt3,
        v1: Pt3,
        v2: Pt3,
        normal: Option<Vec3>,
        material: Arc<dyn Material>,
    ) -> Self {
        let normal = if let Some(normal) = normal {
            normal
        } else {
            let v0v1 = v1 - v0;
            let v0v2 = v2 - v0;
            v0v1.cross(&v0v2)
        }
        .normalize();

        Triangle {
            v0,
            v1,
            v2,
            normal,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = ray.direction.cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < fconsts::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.v0;
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;
        if !utils::is_in_range(t, tmin, tmax) {
            return None;
        }

        let p = ray.point_at_parameter(t);

        Some(HitRecord::new(
            ray,
            t,
            p,
            self.normal,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        // We need this delta because if the triangle is in an axis plane,
        // the bonuding box will be empty in one dimension thus failing
        // its purpose
        let delta = Vec3::repeat(0.1);
        let min = self.v0.inf(&self.v1.inf(&self.v2)) - delta;
        let max = self.v0.sup(&self.v1.sup(&self.v2)) + delta;
        Some(AABB::new(min, max))
    }
}
