use crate::hittable::HitCheckable;
use crate::utils;
use crate::{FloatTy, Ray, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn surrounding(a: AABB, b: AABB) -> AABB {
        let min = Vec3::new(
            utils::fmin(a.min.x, b.min.x),
            utils::fmin(a.min.y, b.min.y),
            utils::fmin(a.min.z, b.min.z),
        );
        let max = Vec3::new(
            utils::fmax(a.max.x, b.max.x),
            utils::fmax(a.max.y, b.max.y),
            utils::fmax(a.max.z, b.max.z),
        );
        AABB { min, max }
    }
}

macro_rules! check_inner_comp {
    ($self:expr, $comp:ident, $ray:expr, $tmin:expr, $tmax:expr) => {
        let inv_dir = 1.0 / $ray.direction.$comp;
        let t0 = ($self.min.$comp - $ray.origin.$comp) * inv_dir;
        let t1 = ($self.max.$comp - $ray.origin.$comp) * inv_dir;
        let (t0, t1) = if inv_dir < 0.0 { (t1, t0) } else { (t0, t1) };
        let tmin = utils::fmax(t0, $tmin);
        let tmax = $tmax.map(|tmax| utils::fmin(t1, tmax)).unwrap_or(t1);
        if tmax <= tmin {
            return false;
        }
    };
}

impl HitCheckable for AABB {
    fn check_hit_by(&self, ray: &Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> bool {
        check_inner_comp!(self, x, ray, tmin, tmax);
        check_inner_comp!(self, y, ray, tmin, tmax);
        check_inner_comp!(self, z, ray, tmin, tmax);
        true
    }
}
