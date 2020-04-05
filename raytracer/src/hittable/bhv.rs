use super::{HitCheckable, HitRecord, Hittable, AABB};
use crate::{FloatTy, Ray};
use rand;
use rand::prelude::*;

pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(left: Box<dyn Hittable>, right: Box<dyn Hittable>) -> Self {
        let bounding_box = match (left.bounding_box(), right.bounding_box()) {
            (Some(lb), Some(rb)) => Some(AABB::surrounding(lb, rb)),
            _ => None,
        };

        BVHNode {
            left,
            right,
            bounding_box: bounding_box.unwrap(),
        }
    }
}

impl Hittable for BVHNode {
    fn is_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        if !self.bounding_box.check_hit_by(ray, tmin, tmax) {
            return None;
        }

        if let Some(hit_left) = self.left.is_hit_by(ray, tmin, tmax) {
            if let Some(hit_right) = self.right.is_hit_by(ray, tmin, Some(hit_left.t)) {
                Some(hit_right)
            } else {
                Some(hit_left)
            }
        } else {
            self.right.is_hit_by(ray, tmin, tmax)
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

// should be call with objects non empty and only objects that contains bounding
// boxes
fn build_bvh_raw<R: Rng>(mut objects: Vec<Box<dyn Hittable>>, rng: &mut R) -> Box<dyn Hittable> {
    assert!(!objects.is_empty());

    if objects.len() == 1 {
        return objects.pop().unwrap();
    }

    let extractor = get_extractor(rng);
    objects.sort_by(|a, b| {
        extractor(a.as_ref())
            .partial_cmp(&extractor(b.as_ref()))
            .unwrap()
    });

    let mid = objects.len() / 2;
    let right_part = objects.split_off(mid);
    let left = build_bvh_raw(objects, rng);
    let right = build_bvh_raw(right_part, rng);

    Box::new(BVHNode::new(left, right))
}

pub fn build_bvh(objects: Vec<Box<dyn Hittable>>) -> Vec<Box<dyn Hittable>> {
    let mut rng = rand::thread_rng();

    let mut bb_objects = Vec::new();
    let mut inf_objects = Vec::new();
    for obj in objects {
        if let Some(_) = obj.bounding_box() {
            bb_objects.push(obj);
        } else {
            inf_objects.push(obj);
        }
    }

    let bvh = build_bvh_raw(bb_objects, &mut rng);
    inf_objects.push(bvh);
    inf_objects
}

type ExtractorFn = fn(&dyn Hittable) -> FloatTy;
fn get_extractor<R: Rng>(rng: &mut R) -> ExtractorFn {
    let index: u32 = rng.gen_range(0, 3);
    match index {
        0 => key_x_extractor,
        1 => key_y_extractor,
        2 => key_z_extractor,
        _ => unreachable!(),
    }
}

fn key_x_extractor(h: &dyn Hittable) -> FloatTy {
    h.bounding_box().unwrap().min.x
}

fn key_y_extractor(h: &dyn Hittable) -> FloatTy {
    h.bounding_box().unwrap().min.x
}

fn key_z_extractor(h: &dyn Hittable) -> FloatTy {
    h.bounding_box().unwrap().min.x
}
