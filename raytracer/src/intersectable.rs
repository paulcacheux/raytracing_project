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
    pub material: Arc<dyn Material>,
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
