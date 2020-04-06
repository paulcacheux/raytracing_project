use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

mod aabb;
mod bhv;
mod plane;
mod rect;
mod sphere;
pub use aabb::*;
pub use bhv::*;
pub use plane::*;
pub use rect::*;
pub use sphere::*;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: FloatTy,
    pub p: Vec3,
    pub normal: Vec3,
    pub u: FloatTy,
    pub v: FloatTy,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
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

        HitRecord {
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
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for &[Box<dyn Hittable>] {
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
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
    fn check_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> bool;
}

impl<T: Hittable> HitCheckable for T {
    fn check_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> bool {
        self.is_hit_by(ray, tmin, tmax).is_some()
    }
}

pub struct FlipFaceHittable<T: Hittable> {
    inner: T,
}

impl<T: Hittable> Hittable for FlipFaceHittable<T> {
    fn bounding_box(&self) -> Option<AABB> {
        self.inner.bounding_box()
    }

    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        if let Some(mut record) = self.inner.is_hit_by(ray, tmin, tmax) {
            record.front_face = !record.front_face;
            Some(record)
        } else {
            None
        }
    }
}

pub trait HittableExt: Hittable + Sized {
    fn flip_face(self) -> FlipFaceHittable<Self> {
        FlipFaceHittable { inner: self }
    }
}

impl<T: Sized + Hittable> HittableExt for T {}
