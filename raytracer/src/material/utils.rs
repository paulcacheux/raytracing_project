use crate::{FloatTy, Vec3};
use rand;
use rand::prelude::*;
use rand_distr::{Distribution, UnitSphere};

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * Vec3::dot(v, n)
}

pub fn refract(uv: Vec3, n: Vec3, n1_over_n2: FloatTy) -> Vec3 {
    let cos_theta = Vec3::dot(-uv, n);
    let r_out_parallel = (uv + n * cos_theta) * n1_over_n2;
    let r_out_perp = n * -(1.0 - r_out_parallel.length_squared()).sqrt();
    r_out_parallel + r_out_perp
}

pub fn schlick(cos: FloatTy, reflective_index: FloatTy) -> FloatTy {
    let r0 = (1.0 - reflective_index) / (1.0 + reflective_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub fn random_unit_hemisphere<R: Rng>(rng: &mut R, normal: Vec3) -> Vec3 {
    let [x, y, z]: [f64; 3] = UnitSphere.sample(rng);
    let v = Vec3::new(x as _, y.abs() as _, z as _).to_unit();

    let t = if normal.x.abs() > normal.y.abs() {
        Vec3::new(normal.z, 0.0, -normal.x).to_unit()
    } else {
        Vec3::new(0.0, -normal.z, normal.y).to_unit()
    };
    let s = Vec3::cross(normal, t);

    Vec3::new(
        v.x * s.x + v.y * normal.x + v.z * t.x,
        v.x * s.y + v.y * normal.y + v.z * t.y,
        v.x * s.z + v.y * normal.z + v.z * t.z,
    )
    .to_unit()
}

pub fn fmin(a: FloatTy, b: FloatTy) -> FloatTy {
    if a <= b {
        a
    } else {
        b
    }
}

/*
pub fn fmax(a: FloatTy, b: FloatTy) -> FloatTy {
    if a >= b {
        a
    } else {
        b
    }
}
*/
