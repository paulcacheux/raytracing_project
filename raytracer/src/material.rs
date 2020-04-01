use crate::intersectable::IntersectionRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

use rand;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct MaterialScatter {
    pub attenuation: Vec3,
    pub scattered: Option<Ray>,
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter>;
}

#[derive(Debug, Clone)]
pub struct Matte {
    albedo: Vec3,
}

impl Matte {
    pub fn new(albedo: Vec3) -> Self {
        Matte { albedo }
    }
}

impl Material for Matte {
    fn scatter(&self, _: &Ray, _: &IntersectionRecord) -> Option<MaterialScatter> {
        Some(MaterialScatter {
            attenuation: self.albedo,
            scattered: None,
        })
    }
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
        // let mut target = record.p + record.normal + random_unit_sphere(&mut rng);
        let new_direction = random_unit_hemisphere(&mut rng, record.normal);

        let scattered = Ray::new(record.p, new_direction);
        Some(MaterialScatter {
            attenuation: self.albedo,
            scattered: Some(scattered),
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

fn random_unit_hemisphere<R: Rng>(rng: &mut R, normal: Vec3) -> Vec3 {
    use crate::vec3::BASES;

    // https://math.stackexchange.com/questions/1585975/how-to-generate-random-points-on-a-sphere
    let u1: f64 = rng.gen();
    let u2: f64 = rng.gen();
    let lambda = (2.0 * u1 - 1.0).acos() - std::f64::consts::FRAC_PI_2;
    let longitude = 2.0 * std::f64::consts::PI * u2;

    let x = lambda.cos() * longitude.cos();
    let y = (lambda.cos() * longitude.sin()).abs(); // abs because we want in the hemisphere
    let z = lambda.sin();

    let v = Vec3::new(x as _, y as _, z as _).to_unit();

    let (comp_index, _) = [v.x, v.y, v.z]
        .iter()
        .map(|comp| comp.abs())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    let s = BASES[comp_index];
    let t = Vec3::cross(normal, s).to_unit();

    Vec3::new(Vec3::dot(v, s), Vec3::dot(v, normal), Vec3::dot(v, t)).to_unit()
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
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let reflected = utils::reflect(ray.direction.to_unit(), record.normal);
        if Vec3::dot(reflected, record.normal) > 0.0 {
            let scattered = Ray::new(
                record.p,
                reflected.to_unit() + random_unit_sphere(&mut rng) * self.fuzz,
            );
            Some(MaterialScatter {
                attenuation: self.albedo,
                scattered: Some(scattered),
            })
        } else {
            None
        }
    }
}

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

mod utils {
    use crate::{FloatTy, Vec3};

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
}
