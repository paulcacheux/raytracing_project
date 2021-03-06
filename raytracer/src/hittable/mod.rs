use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::{FloatTy, Mat44, Pt3, Vec3};

mod aabb;
mod bvh;
mod operation;
mod plane;
mod rect;
mod sphere;
mod triangle;
pub use aabb::*;
pub use bvh::*;
pub use operation::*;
pub use plane::*;
pub use rect::*;
pub use sphere::*;
pub use triangle::*;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub ray: Ray,
    pub t: FloatTy,
    pub p: Pt3,
    pub normal: Vec3,
    pub u: FloatTy,
    pub v: FloatTy,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        ray: Ray,
        t: FloatTy,
        p: Pt3,
        outward_normal: Vec3,
        u: FloatTy,
        v: FloatTy,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            ray,
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

pub trait Hittable: Sync + Send {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        self.as_slice().is_hit_by(ray, tmin, tmax)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.as_slice().bounding_box()
    }
}

impl Hittable for &[Box<dyn Hittable>] {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let mut current_closest = tmax;
        let mut final_record = None;

        for item in *self {
            if let Some(record) = item.is_hit_by(ray, tmin, current_closest) {
                current_closest = Some(record.t);
                final_record = Some(record);
            }
        }

        final_record
    }

    fn bounding_box(&self) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let mut res = if let Some(bb) = self[0].bounding_box() {
            bb.clone()
        } else {
            return None;
        };

        for obj in &self[1..] {
            if let Some(bb) = obj.bounding_box() {
                res = AABB::surrounding(res, bb);
            } else {
                return None;
            }
        }

        Some(res)
    }
}

pub trait HitCheckable: Sync + Send {
    fn check_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> bool;
}

impl<T: Hittable> HitCheckable for T {
    fn check_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> bool {
        self.is_hit_by(ray, tmin, tmax).is_some()
    }
}

pub trait HittableExt: Hittable + Sized {
    fn transform(self, transform: Mat44) -> TransformHittable<Self> {
        TransformHittable::new(self, transform)
    }
}

impl<T: Sized + Hittable> HittableExt for T {}
