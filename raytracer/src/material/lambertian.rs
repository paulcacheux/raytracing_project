use super::{utils, Material, MaterialScatter};
use crate::hittable::HitRecord;
use crate::texture::SolidTexture;
use crate::{Ray, Texture, Vec3};

const HEMISPHERE_MODE: bool = false;

#[derive(Debug)]
pub struct Lambertian<T: Texture> {
    texture: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Lambertian { texture }
    }
}

impl Lambertian<SolidTexture> {
    pub fn from_solid_color(color: Vec3) -> Self {
        Lambertian {
            texture: SolidTexture::new(color),
        }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        let new_direction = if HEMISPHERE_MODE {
            utils::random_unit_hemisphere(&mut rng, record.normal)
        } else {
            utils::random_unit_sphere(&mut rng, record.normal)
        };

        let scattered = Ray::new(record.p, new_direction);
        let attenuation = self.texture.value(record.u, record.v);
        Some(MaterialScatter {
            attenuation,
            scattered,
        })
    }
}
