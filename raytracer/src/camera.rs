use crate::{FloatTy, Pt3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Pt3,
    lower_left: Pt3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Pt3,
        look_at: Pt3,
        vertical_up: Vec3,
        vfov: FloatTy,
        aspect_ratio: FloatTy,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).normalize();
        let u = vertical_up.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            origin: look_from,
            lower_left: look_from - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
        }
    }

    pub fn get_ray(&self, u: FloatTy, v: FloatTy) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
