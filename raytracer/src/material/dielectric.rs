use super::{utils, Material, MaterialScatter};
use crate::hittable::HitRecord;
use crate::utils::fmin;
use crate::{FloatTy, Ray, Vec3};

use rand;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Dielectric {
    reflective_index: FloatTy,
}

impl Dielectric {
    pub fn new(reflective_index: FloatTy) -> Self {
        Dielectric { reflective_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let attenuation = Vec3::repeat(1.0);

        let n1_over_n2 = if record.front_face {
            1.0 / self.reflective_index
        } else {
            self.reflective_index
        };

        let uv = ray.direction.normalize();
        let cos_theta = fmin((-uv).dot(&record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let reflect_prob = utils::schlick(cos_theta, self.reflective_index);

        if n1_over_n2 * sin_theta > 1.0 || rng.gen::<FloatTy>() < reflect_prob {
            let reflected = utils::reflect(uv, record.normal);
            let scattered = Ray::new(record.p, reflected);
            Some(MaterialScatter {
                attenuation,
                scattered: Some(scattered),
            })
        } else {
            let refracted = utils::refract(uv, record.normal, n1_over_n2);
            let scattered = Ray::new(record.p, refracted);
            Some(MaterialScatter {
                attenuation,
                scattered: Some(scattered),
            })
        }
    }
}
