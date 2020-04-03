use std::sync::Arc;

use crate::intersectable::{Intersectable, IntersectionRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::Vec3;
use crate::FloatTy;

#[derive(Debug, Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Intersectable for Plane {
    fn is_intersected_by(
        &self,
        ray: &Ray,
        tmin: FloatTy,
        tmax: Option<FloatTy>,
    ) -> Option<IntersectionRecord> {
        let denominator = Vec3::dot(ray.direction, self.normal);
        if denominator.abs() <= std::f32::EPSILON {
            return None;
        }

        let t = Vec3::dot(self.point - ray.origin, self.normal) / denominator;
        if utils::is_in_range(t, tmin, tmax) {
            let p = ray.point_at_parameter(t);
            Some(IntersectionRecord {
                t,
                p,
                // TODO: fix this
                u: 0.0,
                v: 0.0,
                normal: self.normal,
                material: self.material.clone(),
            })
        } else {
            None
        }
    }
}
