use super::{HitRecord, Hittable, AABB};
use crate::{FloatTy, Mat44, Ray, Vec3};

pub struct FlipFaceHittable<H: Hittable> {
    inner: H,
}

impl<H: Hittable> FlipFaceHittable<H> {
    pub fn new(inner: H) -> Self {
        FlipFaceHittable { inner }
    }
}

impl<H: Hittable> Hittable for FlipFaceHittable<H> {
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

pub struct TransformHittable<H: Hittable> {
    inner: H,
    transform: Mat44,
    inverse: Mat44,
}

impl<H: Hittable> TransformHittable<H> {
    pub fn new(inner: H, transform: Mat44) -> Self {
        let inverse = transform.inverse();
        TransformHittable {
            inner,
            transform,
            inverse,
        }
    }
}

impl<H: Hittable> Hittable for TransformHittable<H> {
    fn bounding_box(&self) -> Option<AABB> {
        let aabb = if let Some(aabb) = self.inner.bounding_box() {
            aabb
        } else {
            return None;
        };

        let mut min_x = None;
        let mut min_y = None;
        let mut min_z = None;
        let mut max_x = None;
        let mut max_y = None;
        let mut max_z = None;

        let points = [aabb.min, aabb.max];
        for dx in &points {
            for dy in &points {
                for dz in &points {
                    let corner = Vec3::new(dx.x, dy.y, dz.z);
                    let trans_corner = self.transform.mul_point(corner);

                    if min_x.map(|m| trans_corner.x < m).unwrap_or(true) {
                        min_x = Some(trans_corner.x);
                    }

                    if min_y.map(|m| trans_corner.y < m).unwrap_or(true) {
                        min_y = Some(trans_corner.y);
                    }

                    if min_z.map(|m| trans_corner.z < m).unwrap_or(true) {
                        min_z = Some(trans_corner.z);
                    }

                    if max_x.map(|m| trans_corner.x > m).unwrap_or(true) {
                        max_x = Some(trans_corner.x);
                    }

                    if max_y.map(|m| trans_corner.y > m).unwrap_or(true) {
                        max_y = Some(trans_corner.y);
                    }

                    if max_z.map(|m| trans_corner.z > m).unwrap_or(true) {
                        max_z = Some(trans_corner.z);
                    }
                }
            }
        }

        let min = Vec3::new(min_x.unwrap(), min_y.unwrap(), min_z.unwrap());
        let max = Vec3::new(max_x.unwrap(), max_y.unwrap(), max_z.unwrap());

        Some(AABB::new(min, max))
    }

    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let new_ray = Ray::new(
            self.inverse.mul_point(ray.origin),
            self.inverse.mul_direction(ray.direction),
        );

        if let Some(record) = self.inner.is_hit_by(&new_ray, tmin, tmax) {
            Some(HitRecord {
                p: self.transform.mul_point(record.p),
                normal: self.transform.mul_direction(record.normal),
                ..record
            })
        } else {
            None
        }
    }
}
