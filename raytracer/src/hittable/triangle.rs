use std::sync::Arc;

use super::{HitRecord, Hittable, AABB};
use crate::fconsts;
use crate::material::Material;
use crate::utils;
use crate::{FloatTy, Pt3, Ray, Vec3};

type TexCoords = [FloatTy; 2];

pub struct TriangleBuilder {
    points: [Pt3; 3],
    normals: Option<[Vec3; 3]>,
    texcoords: Option<[TexCoords; 3]>,
    material: Arc<dyn Material>,
}

impl TriangleBuilder {
    pub fn new(points: [Pt3; 3], material: Arc<dyn Material>) -> Self {
        TriangleBuilder {
            points,
            normals: None,
            texcoords: None,
            material,
        }
    }

    pub fn with_normals(self, normals: [Vec3; 3]) -> Self {
        TriangleBuilder {
            normals: Some(normals),
            ..self
        }
    }

    pub fn with_texcoords(self, texcoords: [TexCoords; 3]) -> Self {
        TriangleBuilder {
            texcoords: Some(texcoords),
            ..self
        }
    }

    pub fn build(self) -> Triangle {
        let v0 = self.points[0];
        let v1 = self.points[1];
        let v2 = self.points[2];

        let normal = if let Some(normals) = self.normals {
            TriangleNormal::Barycentric(
                normals[0].normalize(),
                normals[1].normalize(),
                normals[2].normalize(),
            )
        } else {
            let v0v1 = v1 - v0;
            let v0v2 = v2 - v0;
            TriangleNormal::Uniform(v0v1.cross(&v0v2).normalize())
        };

        let texcoords = if let Some(coords) = self.texcoords {
            TriangleTexCoords::Barycentric(coords[0], coords[1], coords[2])
        } else {
            TriangleTexCoords::NoTexture
        };

        Triangle {
            v0,
            v1,
            v2,
            normal,
            texcoords,
            material: self.material,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TriangleNormal {
    Uniform(Vec3),
    Barycentric(Vec3, Vec3, Vec3),
}

impl TriangleNormal {
    pub fn compute_normal(&self, u: FloatTy, v: FloatTy) -> Vec3 {
        match self {
            TriangleNormal::Uniform(n) => *n,
            TriangleNormal::Barycentric(a, b, c) => {
                let w = 1.0 - u - v;
                a * w + b * u + c * v
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TriangleTexCoords {
    NoTexture,
    Barycentric(TexCoords, TexCoords, TexCoords),
}

impl TriangleTexCoords {
    pub fn compute_uv(&self, u: FloatTy, v: FloatTy) -> (FloatTy, FloatTy) {
        match self {
            TriangleTexCoords::NoTexture => (u, v),
            TriangleTexCoords::Barycentric(a, b, c) => {
                let w = 1.0 - u - v;
                let tu = a[0] * w + b[0] * u + c[0] * v;
                let tv = a[1] * w + b[1] * u + c[1] * v;
                (tu, tv)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub v0: Pt3,
    pub v1: Pt3,
    pub v2: Pt3,
    pub normal: TriangleNormal,
    pub texcoords: TriangleTexCoords,
    pub material: Arc<dyn Material>,
}

impl Hittable for Triangle {
    fn is_hit_by(&self, ray: Ray, tmin: FloatTy, tmax: Option<FloatTy>) -> Option<HitRecord> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = ray.direction.cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < fconsts::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.v0;
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;
        if !utils::is_in_range(t, tmin, tmax) {
            return None;
        }

        let p = ray.point_at_parameter(t);

        let normal = self.normal.compute_normal(u, v);
        let (u, v) = self.texcoords.compute_uv(u, v);

        Some(HitRecord::new(
            ray,
            t,
            p,
            normal,
            u,
            v,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        // We need this delta because if the triangle is in an axis plane,
        // the bonuding box will be empty in one dimension thus failing
        // its purpose
        let delta = Vec3::repeat(0.1);
        let min = self.v0.inf(&self.v1.inf(&self.v2)) - delta;
        let max = self.v0.sup(&self.v1.sup(&self.v2)) + delta;
        Some(AABB::new(min, max))
    }
}
