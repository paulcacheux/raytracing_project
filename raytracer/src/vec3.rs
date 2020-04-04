use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::FloatTy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: FloatTy,
    pub y: FloatTy,
    pub z: FloatTy,
}

impl Vec3 {
    pub const fn new(x: FloatTy, y: FloatTy, z: FloatTy) -> Self {
        Vec3 { x, y, z }
    }

    pub const fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub const fn all(x: FloatTy) -> Self {
        Vec3::new(x, x, x)
    }

    pub fn dot(a: Self, b: Self) -> FloatTy {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn memberwise_product(a: Self, b: Self) -> Self {
        Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
    }

    pub fn length_squared(&self) -> FloatTy {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> FloatTy {
        self.length_squared().sqrt()
    }

    pub fn to_unit(self) -> Self {
        self / self.length()
    }
}

macro_rules! impl_from_array_vec3 {
    ($float_ty:ty) => {
        impl From<[$float_ty; 3]> for Vec3 {
            fn from(array: [$float_ty; 3]) -> Self {
                Vec3 {
                    x: array[0] as _,
                    y: array[1] as _,
                    z: array[2] as _,
                }
            }
        }
    };
}

impl_from_array_vec3!(f32);
impl_from_array_vec3!(f64);

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul<FloatTy> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: FloatTy) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl std::ops::MulAssign<FloatTy> for Vec3 {
    fn mul_assign(&mut self, other: FloatTy) {
        *self = *self * other
    }
}

impl std::ops::Div<FloatTy> for Vec3 {
    type Output = Vec3;

    fn div(self, other: FloatTy) -> Self {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl std::ops::DivAssign<FloatTy> for Vec3 {
    fn div_assign(&mut self, other: FloatTy) {
        *self = *self / other
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }
}
