use crate::intersectable::IntersectionRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

use rand;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct MaterialScatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter>;
}

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
    fn scatter(&self, _: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let target = record.p + record.normal + random_unit_sphere(&mut rng);
        let scattered = Ray::new(record.p, target - record.p);
        Some(MaterialScatter {
            attenuation: self.albedo,
            scattered,
        })
    }
}

fn random_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let temp = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::all(1.0);
        if temp.length_squared() < 1.0 {
            return temp;
        }
    }
}

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

    fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2.0 * Vec3::dot(v, n)
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let reflected = Metal::reflect(ray.direction.to_unit(), record.normal);
        if Vec3::dot(reflected, record.normal) > 0.0 {
            let scattered = Ray::new(
                record.p,
                reflected.to_unit() + random_unit_sphere(&mut rng) * self.fuzz,
            );
            Some(MaterialScatter {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
