use crate::FloatTy;
use crate::{Pt3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Pt3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Pt3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, param: FloatTy) -> Pt3 {
        self.origin + self.direction * param
    }
}
