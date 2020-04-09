use crate::{FloatTy, Vec3};
use rand;
use rand::prelude::*;
use rand_distr::{Distribution, UnitSphere};

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - v.dot(&n) * 2.0 * n
}

#[inline]
pub fn refract(uv: Vec3, n: Vec3, n1_over_n2: FloatTy) -> Vec3 {
    let cos_theta = (-uv).dot(&n);
    let r_out_parallel = (uv + n * cos_theta) * n1_over_n2;
    let r_out_perp = n * -(1.0 - r_out_parallel.norm_squared()).sqrt();
    r_out_parallel + r_out_perp
}

#[inline]
pub fn schlick(cos: FloatTy, reflective_index: FloatTy) -> FloatTy {
    let r0 = (1.0 - reflective_index) / (1.0 + reflective_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[inline]
pub fn random_unit_hemisphere<R: Rng>(rng: &mut R, normal: Vec3) -> Vec3 {
    let v: [FloatTy; 3] = UnitSphere.sample(rng);
    let v: Vec3 = v.into();
    if v.dot(&normal) > 0.0 {
        v
    } else {
        v
    }
}

#[inline]
pub fn random_unit_sphere<R: Rng>(rng: &mut R, normal: Vec3) -> Vec3 {
    let v: [FloatTy; 3] = UnitSphere.sample(rng);
    let v: Vec3 = v.into();
    v + normal
}
