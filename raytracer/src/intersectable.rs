use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

#[derive(Debug, Clone)]
pub struct IntersectionRecord {
    pub t: FloatTy,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: FloatTy,
    pub v: FloatTy,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl IntersectionRecord {
    pub fn new(
        ray: &Ray,
        t: FloatTy,
        p: Vec3,
        outward_normal: Vec3,
        u: FloatTy,
        v: FloatTy,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        IntersectionRecord {
            t,
            p,
            normal,
            u,
            v,
            front_face,
            material,
        }
    }
}

pub trait Intersectable: Sync + Send {
    fn is_intersected_by(
        &self,
        ray: &Ray,
        tmin: FloatTy,
        tmax: Option<FloatTy>,
    ) -> Option<IntersectionRecord>;
}

pub type IntersectableList = Vec<Box<dyn Intersectable>>;

impl Intersectable for &[Box<dyn Intersectable>] {
    fn is_intersected_by(
        &self,
        ray: &Ray,
        tmin: FloatTy,
        tmax: Option<FloatTy>,
    ) -> Option<IntersectionRecord> {
        let mut current_closest = tmax;
        let mut final_record = None;

        for item in *self {
            if let Some(record) = item.is_intersected_by(ray, tmin, current_closest) {
                current_closest = Some(record.t);
                final_record = Some(record);
            }
        }

        final_record
    }
}
