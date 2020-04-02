use crate::intersectable::IntersectionRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

use rand;
use rand::prelude::*;
use rand_distr::{Distribution, UnitSphere};

#[derive(Debug, Clone)]
pub struct MaterialScatter {
    pub attenuation: Vec3,
    pub scattered: Option<Ray>,
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter>;
    fn emit(&self, _u: FloatTy, _v: FloatTy, _point: Vec3) -> Vec3 {
        Vec3::all(0.0)
    }
}

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
        let new_direction = utils::random_unit_hemisphere(&mut rng, record.normal);

        let scattered = Ray::new(record.p, new_direction);
        Some(MaterialScatter {
            attenuation: self.albedo,
            scattered: Some(scattered),
        })
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
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &IntersectionRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let reflected = utils::reflect(ray.direction.to_unit(), record.normal);
        if Vec3::dot(reflected, record.normal) > 0.0 {
            let sample_sphere: [FloatTy; 3] = UnitSphere.sample(&mut rng);
            let sample_sphere: Vec3 = sample_sphere.into();
            let scattered = Ray::new(record.p, reflected.to_unit() + sample_sphere * self.fuzz);
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
}
