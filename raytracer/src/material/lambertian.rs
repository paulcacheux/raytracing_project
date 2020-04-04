use super::{utils, Material, MaterialScatter};
use crate::hittable::HitRecord;
use crate::{Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let new_direction = utils::random_unit_hemisphere(&mut rng, record.normal);

        let scattered = Ray::new(record.p, new_direction);
        Some(MaterialScatter {
            attenuation: self.albedo,
            scattered: Some(scattered),
        })
    }
}
