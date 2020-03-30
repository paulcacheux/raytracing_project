use crate::math::{FloatTy, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, param: FloatTy) -> FloatTy {
        self.origin + self.direction * param
    }
}
