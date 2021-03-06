use super::{utils, Material, MaterialScatter};
use crate::hittable::HitRecord;
use crate::{FloatTy, Ray, Vec3};

use rand_distr::{Distribution, UnitSphere};

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: FloatTy,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: Option<FloatTy>) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.unwrap_or(0.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let reflected = utils::reflect(ray.direction.normalize(), record.normal);
        let scattered = if reflected.dot(&record.normal) > 0.0 {
            let sample_sphere: [FloatTy; 3] = UnitSphere.sample(&mut rng);
            let sample_sphere: Vec3 = sample_sphere.into();
            let scattered = Ray::new(record.p, reflected.normalize() + sample_sphere * self.fuzz);
            Some(scattered)
        } else {
            None
        };
        Some(MaterialScatter {
            attenuation: self.albedo,
            scattered,
        })
    }
}
