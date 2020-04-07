use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable, HittableExt, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::Vec3;
use crate::FloatTy;

const DELTA: FloatTy = 0.001;

#[derive(Debug, Clone)]
pub struct XYRect {
    pub x0: FloatTy,
    pub x1: FloatTy,
    pub y0: FloatTy,
    pub y1: FloatTy,
    pub z: FloatTy,
    pub material: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(
        x0: FloatTy,
        x1: FloatTy,
        y0: FloatTy,
        y1: FloatTy,
        z: FloatTy,
        material: Arc<dyn Material>,
    ) -> Self {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            z,
            material,
        }
    }
}

impl Hittable for XYRect {
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / ray.direction.z;
        if !utils::is_in_range(t, tmin, tmax) {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let p = ray.point_at_parameter(t);

        Some(HitRecord::new(
            ray,
            t,
            p,
            outward_normal,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.z - DELTA),
            Vec3::new(self.x1, self.y1, self.z + DELTA),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct YZRect {
    pub x: FloatTy,
    pub y0: FloatTy,
    pub y1: FloatTy,
    pub z0: FloatTy,
    pub z1: FloatTy,
    pub material: Arc<dyn Material>,
}

impl YZRect {
    pub fn new(
        y0: FloatTy,
        y1: FloatTy,
        z0: FloatTy,
        z1: FloatTy,
        x: FloatTy,
        material: Arc<dyn Material>,
    ) -> Self {
        YZRect {
            x,
            y0,
            y1,
            z0,
            z1,
            material,
        }
    }
}

impl Hittable for YZRect {
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x) / ray.direction.x;
        if !utils::is_in_range(t, tmin, tmax) {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;

        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let p = ray.point_at_parameter(t);

        Some(HitRecord::new(
            ray,
            t,
            p,
            outward_normal,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x - DELTA, self.y0, self.z0),
            Vec3::new(self.x + DELTA, self.y1, self.z1),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct XZRect {
    pub x0: FloatTy,
    pub x1: FloatTy,
    pub z0: FloatTy,
    pub z1: FloatTy,
    pub y: FloatTy,
    pub material: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(
        x0: FloatTy,
        x1: FloatTy,
        z0: FloatTy,
        z1: FloatTy,
        y: FloatTy,
        material: Arc<dyn Material>,
    ) -> Self {
        XZRect {
            x0,
            x1,
            y,
            z0,
            z1,
            material,
        }
    }
}

impl Hittable for XZRect {
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let t = (self.y - ray.origin.y) / ray.direction.y;
        if !utils::is_in_range(t, tmin, tmax) {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = ray.point_at_parameter(t);

        Some(HitRecord::new(
            ray,
            t,
            p,
            outward_normal,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y - DELTA, self.z0),
            Vec3::new(self.x1, self.y + DELTA, self.z1),
        ))
    }
}

pub fn make_box(min: Vec3, max: Vec3, material: Arc<dyn Material>) -> Vec<Box<dyn Hittable>> {
    let mut res: Vec<Box<dyn Hittable>> = Vec::with_capacity(6);

    res.push(Box::new(XYRect::new(
        min.x,
        max.x,
        min.y,
        max.y,
        max.z,
        material.clone(),
    )));

    res.push(Box::new(
        XYRect::new(min.x, max.x, min.y, max.y, min.z, material.clone()).flip_face(),
    ));

    res.push(Box::new(XZRect::new(
        min.x,
        max.x,
        min.z,
        max.z,
        max.y,
        material.clone(),
    )));

    res.push(Box::new(
        XZRect::new(min.x, max.x, min.z, max.z, min.y, material.clone()).flip_face(),
    ));

    res.push(Box::new(YZRect::new(
        min.y,
        max.y,
        min.z,
        max.z,
        max.x,
        material.clone(),
    )));

    res.push(Box::new(
        YZRect::new(min.y, max.y, min.z, max.z, min.x, material.clone()).flip_face(),
    ));

    res
}
