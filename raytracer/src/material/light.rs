use super::{Material, MaterialScatter};
use crate::{FloatTy, IntersectionRecord, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Light {
    emittance: Vec3,
}

impl Light {
    pub fn white() -> Self {
        Light {
            emittance: Vec3::all(1.0),
        }
    }

    pub fn new(emittance: Vec3) -> Self {
        Light { emittance }
    }
}

impl Material for Light {
    fn scatter(&self, _: &Ray, _: &IntersectionRecord) -> Option<MaterialScatter> {
        Some(MaterialScatter {
            attenuation: Vec3::all(0.0),
            scattered: None,
        })
    }

    fn emit(&self, _u: FloatTy, _v: FloatTy, _point: Vec3) -> Vec3 {
        self.emittance
    }
}
