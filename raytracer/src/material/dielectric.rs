use super::{utils, Material, MaterialScatter};
use crate::{FloatTy, IntersectionRecord, Ray, Vec3};

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
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let attenuation = Vec3::all(1.0);

        let dir_dot_normal = Vec3::dot(ray.direction, record.normal);

        let (outward_normal, ni_over_nt, cos) = if dir_dot_normal >= 0.0 {
            (
                -record.normal,
                self.reflective_index,
                self.reflective_index * dir_dot_normal / ray.direction.length(),
            )
        } else {
            (
                record.normal,
                1.0 / self.reflective_index,
                -dir_dot_normal / ray.direction.length(),
            )
        };

        let reflected = utils::reflect(ray.direction, record.normal);
        let scattered =
            if let Some(refracted) = utils::refract(ray.direction, outward_normal, ni_over_nt) {
                let reflect_prob = utils::schlick(cos, self.reflective_index);
                if rng.gen::<FloatTy>() < reflect_prob {
                    Ray::new(record.p, reflected)
                } else {
                    Ray::new(record.p, refracted)
                }
            } else {
                Ray::new(record.p, reflected)
            };

        Some(MaterialScatter {
            attenuation,
            scattered: Some(scattered),
        })
    }
}
