use crate::{FloatTy, Vec3};
use rand;
use rand::prelude::*;
use rand_distr::{Distribution, UnitSphere};

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * 2.0 * Vec3::dot(v, n)
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: FloatTy) -> Option<Vec3> {
    let uv = v.to_unit();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
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
