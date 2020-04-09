use crate::{FloatTy, Pt3, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Mat44 {
    inner: [[FloatTy; 4]; 4],
}

impl Mat44 {
    pub fn identity() -> Self {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    pub fn translation(by: Vec3) -> Self {
        [
            [1.0, 0.0, 0.0, by.x],
            [0.0, 1.0, 0.0, by.y],
            [0.0, 0.0, 1.0, by.z],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    pub fn rotation(axis: Vec3, angle: FloatTy) -> Self {
        let axis = axis.normalize();
        let c = angle.cos();
        let s = angle.sin();
        let t = 1.0 - c;
        [
            [
                c + axis.x * axis.x * t,
                axis.x * axis.y * t - axis.z * s,
                axis.x * axis.z * t + axis.y * s,
                0.0,
            ],
            [
                axis.y * axis.x * t + axis.z * s,
                c + axis.y * axis.y * t,
                axis.y * axis.z * t - axis.x * s,
                0.0,
            ],
            [
                axis.z * axis.x * t - axis.y * s,
                axis.z * axis.y * t + axis.x * s,
                c + axis.z * axis.z * t,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }

    pub fn mul_point(&self, point: Pt3) -> Pt3 {
        let other = [point.x, point.y, point.z, 1.0];
        let mut res = [0.0; 3];
        for i in 0..3 {
            let mut value = 0.0;
            for c in 0..4 {
                value += self.inner[i][c] * other[c];
            }
            res[i] = value;
        }
        res.into()
    }

    pub fn mul_direction(&self, dir: Vec3) -> Vec3 {
        let other = [dir.x, dir.y, dir.z];
        let mut res = [0.0; 3];
        for i in 0..3 {
            let mut value = 0.0;
            for c in 0..3 {
                value += self.inner[i][c] * other[c];
            }
            res[i] = value;
        }
        res.into()
    }

    pub fn inverse(&self) -> Mat44 {
        let mut dst = Mat44::identity();
        let mut tmp = self.clone();

        for i in 0..4 {
            let mut val = tmp.inner[i][i];
            let mut ind = i;
            for j in i + 1..4 {
                if tmp.inner[i][j].abs() > val.abs() {
                    ind = j;
                    val = tmp.inner[i][j];
                }
            }

            if ind != i {
                // Swap columns
                for j in 0..4 {
                    dst.inner[j].swap(i, ind);
                    tmp.inner[j].swap(i, ind);
                }
            }

            if val.abs() < 1e-6 {
                panic!("Matrix not inversable");
            }

            let ival = 1.0 / val;
            for j in 0..4 {
                tmp.inner[j][i] *= ival;
                dst.inner[j][i] *= ival;
            }

            for j in 0..4 {
                if j == i {
                    continue;
                }

                val = tmp.inner[i][j];
                for k in 0..4 {
                    tmp.inner[k][j] -= tmp.inner[k][i] * val;
                    dst.inner[k][j] -= dst.inner[k][i] * val;
                }
            }
        }

        dst
    }
}

impl From<[[FloatTy; 4]; 4]> for Mat44 {
    fn from(raw: [[FloatTy; 4]; 4]) -> Mat44 {
        Mat44 { inner: raw }
    }
}

impl std::ops::Add<Mat44> for Mat44 {
    type Output = Mat44;

    fn add(self, other: Mat44) -> Mat44 {
        let mut res = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = self.inner[i][j] + other.inner[i][j];
            }
        }
        res.into()
    }
}

impl std::ops::Mul<Mat44> for Mat44 {
    type Output = Mat44;

    fn mul(self, other: Mat44) -> Mat44 {
        let mut res = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut value = 0.0;
                for c in 0..4 {
                    value += self.inner[i][c] * other.inner[c][j];
                }
                res[i][j] = value;
            }
        }
        res.into()
    }
}
