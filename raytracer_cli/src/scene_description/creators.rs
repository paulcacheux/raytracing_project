use std::collections::HashMap;
use std::sync::Arc;

use crate::scene_description::{SceneDescription, SceneObject};

use raytracer::{Lambertian, Light, Material, Metal, Plane, Sphere, Vec3};

macro_rules! unwrap_scene_object {
    ($params:expr; $name:expr; $object_desc:pat => $value:expr) => {
        match $params.get($name).unwrap() {
            $object_desc => $value,
            _ => unreachable!(),
        }
    };
}

macro_rules! optional_scene_object {
    ($params:expr; $name:expr; $object_desc:pat => $value:expr) => {
        match $params.get($name) {
            Some($object_desc) => Some($value),
            Some(_) => unreachable!(),
            None => None,
        }
    };
}

pub(crate) fn sphere_creator(desc: &mut SceneDescription, params: HashMap<String, SceneObject>) {
    let center: Vec3 = unwrap_scene_object!(params; "center"; SceneObject::Vec3(v) => *v);
    let radius: f32 = unwrap_scene_object!(params; "radius"; SceneObject::Float(f) => *f);
    let material: Arc<dyn Material> =
        unwrap_scene_object!(params; "material"; SceneObject::Material(m) => m.clone());

    desc.declarations
        .push(Box::new(Sphere::new(center, radius, material)));
}

pub(crate) fn plane_creator(desc: &mut SceneDescription, params: HashMap<String, SceneObject>) {
    let point: Vec3 = unwrap_scene_object!(params; "point"; SceneObject::Vec3(v) => *v);
    let normal: Vec3 = unwrap_scene_object!(params; "normal"; SceneObject::Vec3(v) => *v);
    let material: Arc<dyn Material> =
        unwrap_scene_object!(params; "material"; SceneObject::Material(m) => m.clone());

    desc.declarations
        .push(Box::new(Plane::new(point, normal, material)));
}

pub(crate) fn lambertian_creator(params: HashMap<String, SceneObject>) -> SceneObject {
    let albedo: Vec3 = unwrap_scene_object!(params; "albedo"; SceneObject::Vec3(v) => *v);
    SceneObject::Material(Arc::new(Lambertian::new(albedo)))
}

pub(crate) fn light_creator(params: HashMap<String, SceneObject>) -> SceneObject {
    let emittance: Option<Vec3> =
        optional_scene_object!(params; "emittance"; SceneObject::Vec3(v) => *v);
    let emittance = emittance.unwrap_or(Vec3::all(1.0));
    SceneObject::Material(Arc::new(Light::new(emittance)))
}

pub(crate) fn metal_creator(params: HashMap<String, SceneObject>) -> SceneObject {
    let albedo: Vec3 = unwrap_scene_object!(params; "albedo"; SceneObject::Vec3(v) => *v);
    let fuzz: Option<f32> = optional_scene_object!(params; "fuzz"; SceneObject::Float(f) => *f);
    SceneObject::Material(Arc::new(Metal::new(albedo, fuzz)))
}
