use super::{Material, MaterialScatter};
use crate::hittable::HitRecord;
use crate::{FloatTy, Pt3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Light {
    emittance: Vec3,
}

impl Light {
    pub fn white() -> Self {
        Light {
            emittance: Vec3::repeat(1.0),
        }
    }

    pub fn new(emittance: Vec3) -> Self {
        Light { emittance }
    }
}

impl Material for Light {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<MaterialScatter> {
        None
    }

    fn emit(&self, _u: FloatTy, _v: FloatTy, _point: Pt3) -> Vec3 {
        self.emittance
    }
}
