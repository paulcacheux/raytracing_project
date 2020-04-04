use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::scene_description::{SceneObject, SceneObjectKind};
use crate::PresetConfig;

use raytracer::material::{Lambertian, Light, Material, Metal};
use raytracer::{Intersectable, Plane, Sphere, Vec3};

pub(crate) fn typecheck_params(
    prototype: &HashMap<String, (SceneObjectKind, bool)>,
    params: &HashMap<String, SceneObject>,
) {
    for (arg_name, (arg_kind, optional)) in prototype {
        match (optional, params.get(arg_name)) {
            (_, Some(param)) => {
                if param.kind() != *arg_kind {
                    panic!("TYPE MISMATCH: {}", arg_name);
                }
            }
            (true, None) => {
                panic!("ARG MISSING: {}", arg_name);
            }
            (false, None) => {}
        }
    }

    // check extraneous params
    let proto_params: HashSet<&str> = prototype.keys().map(|s| s.as_ref()).collect();
    let param_names: HashSet<&str> = params.keys().map(|s| s.as_ref()).collect();
    if !param_names.is_subset(&proto_params) {
        panic!("EXTRANEOUS PARAMETERS");
    }
}

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

pub(crate) fn preset_creator(params: HashMap<String, SceneObject>) -> PresetConfig {
    let width = unwrap_scene_object!(params; "width"; SceneObject::Float(f) => *f as usize);
    let height = unwrap_scene_object!(params; "height"; SceneObject::Float(f) => *f as usize);
    let sample_count =
        unwrap_scene_object!(params; "sample_count"; SceneObject::Float(f) => *f as usize);
    let look_from = unwrap_scene_object!(params; "look_from"; SceneObject::Vec3(v) => *v);
    let look_at = unwrap_scene_object!(params; "look_at"; SceneObject::Vec3(v) => *v);
    let up = unwrap_scene_object!(params; "up"; SceneObject::Vec3(v) => *v);
    let vfov = unwrap_scene_object!(params; "vfov"; SceneObject::Float(f) => *f);
    let max_depth = unwrap_scene_object!(params; "max_depth"; SceneObject::Float(f) => *f as usize);
    let background = optional_scene_object!(params; "background"; SceneObject::Vec3(v) => *v);

    PresetConfig {
        width,
        height,
        sample_count,
        look_from,
        look_at,
        up,
        vfov,
        max_depth,
        background,
    }
}

pub(crate) fn sphere_creator(params: HashMap<String, SceneObject>) -> Box<dyn Intersectable> {
    let center: Vec3 = unwrap_scene_object!(params; "center"; SceneObject::Vec3(v) => *v);
    let radius: f32 = unwrap_scene_object!(params; "radius"; SceneObject::Float(f) => *f);
    let material: Arc<dyn Material> =
        unwrap_scene_object!(params; "material"; SceneObject::Material(m) => m.clone());

    Box::new(Sphere::new(center, radius, material))
}

pub(crate) fn plane_creator(params: HashMap<String, SceneObject>) -> Box<dyn Intersectable> {
    let point: Vec3 = unwrap_scene_object!(params; "point"; SceneObject::Vec3(v) => *v);
    let normal: Vec3 = unwrap_scene_object!(params; "normal"; SceneObject::Vec3(v) => *v);
    let material: Arc<dyn Material> =
        unwrap_scene_object!(params; "material"; SceneObject::Material(m) => m.clone());

    Box::new(Plane::new(point, normal, material))
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
