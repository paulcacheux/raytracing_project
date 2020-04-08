use crate::vec3::Vec3;
use crate::FloatTy;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, param: FloatTy) -> Vec3 {
        self.origin + self.direction * param
    }
}
