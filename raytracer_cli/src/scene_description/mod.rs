use std::collections::HashMap;
use std::sync::Arc;

use crate::PresetConfig;
use raytracer::material::Material;
use raytracer::{Hittable, Vec3};

pub mod builder;
mod creators;
pub use builder::SceneDescriptionBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneObjectKind {
    Float,
    Vec3,
    Material,
}

#[derive(Clone)]
pub enum SceneObject {
    Float(f32),
    Vec3(Vec3),
    Material(Arc<dyn Material>),
}

impl SceneObject {
    pub fn kind(&self) -> SceneObjectKind {
        match self {
            SceneObject::Float(_) => SceneObjectKind::Float,
            SceneObject::Vec3(_) => SceneObjectKind::Vec3,
            SceneObject::Material(_) => SceneObjectKind::Material,
        }
    }
}

pub enum SceneObjectOrIdentifier {
    SceneObject(SceneObject),
    Identifier(String),
}

pub struct SceneDescription {
    pub declarations: Vec<Box<dyn Hittable>>,
    pub presets: HashMap<String, PresetConfig>,
}
