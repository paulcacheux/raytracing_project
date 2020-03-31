use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::FloatTy;

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: FloatTy) -> Self {
        Camera {
            origin: Vec3::zero(),
            lower_left: Vec3::new(-1.0, -1.0, -1.0),
            horizontal: Vec3::new(2.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 / aspect_ratio, 0.0),
        }
    }

    pub fn get_ray(&self, u: FloatTy, v: FloatTy) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v,
        )
    }
}
