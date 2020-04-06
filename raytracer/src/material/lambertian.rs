use std::sync::Arc;

use super::{utils, Material, MaterialScatter};
use crate::hittable::HitRecord;
use crate::texture::SolidTexture;
use crate::{Ray, Texture, Vec3};

#[derive(Debug)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Lambertian { texture }
    }

    pub fn from_solid_color(color: Vec3) -> Self {
        Lambertian::new(Arc::new(SolidTexture::new(color)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<MaterialScatter> {
        let mut rng = rand::thread_rng();
        // let new_direction = utils::random_unit_hemisphere(&mut rng, record.normal);
        let new_direction = utils::random_unit_vector(&mut rng, record.normal);

        let scattered = Ray::new(record.p, new_direction);
        let attenuation = self.texture.value(record.u, record.v);
        Some(MaterialScatter {
            attenuation,
            scattered: Some(scattered),
        })
    }
}
