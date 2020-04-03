use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use lazy_static::lazy_static;
use maplit::hashmap;

use raytracer::{Intersectable, Material, Vec3};

mod creators;

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

#[derive(Default)]
pub struct SceneDescription {
    variables: HashMap<String, SceneObject>,
    declarations: Vec<Box<dyn Intersectable>>,
}

type DeclCreatorFn = fn(&mut SceneDescription, params: HashMap<String, SceneObject>);
type BuildCreatorFn = fn(params: HashMap<String, SceneObject>) -> SceneObject;

lazy_static! {
    static ref DECLARABLES: HashMap<String, (HashMap<String, (SceneObjectKind, bool)>, DeclCreatorFn)> = {
        let mut map = HashMap::new();
        map.insert(
            "sphere".to_string(),
            (
                hashmap! {
                    "center".to_string() => (SceneObjectKind::Vec3, true),
                    "radius".into() => (SceneObjectKind::Float, true),
                    "material".into() => (SceneObjectKind::Material, true),
                },
                creators::sphere_creator as _,
            ),
        );

        map.insert(
            "plane".to_string(),
            (
                hashmap! {
                    "point".to_string() => (SceneObjectKind::Vec3, true),
                    "normal".into() => (SceneObjectKind::Vec3, true),
                    "material".into() => (SceneObjectKind::Material, true),
                },
                creators::plane_creator as _,
            ),
        );

        map
    };
    static ref BUILDABLES: HashMap<String, (HashMap<String, (SceneObjectKind, bool)>, BuildCreatorFn)> = {
        let mut map = HashMap::new();
        map.insert(
            "lambertian".to_string(),
            (
                hashmap! {
                    "albedo".to_string() => (SceneObjectKind::Vec3, true),
                },
                creators::lambertian_creator as _,
            ),
        );

        map.insert(
            "light".to_string(),
            (
                hashmap! {
                    "emittance".to_string() => (SceneObjectKind::Vec3, false),
                },
                creators::light_creator as _,
            ),
        );

        map.insert(
            "metal".to_string(),
            (
                hashmap! {
                    "albedo".to_string() => (SceneObjectKind::Vec3, true),
                    "fuzz".to_string() => (SceneObjectKind::Float, false),
                },
                creators::metal_creator as _,
            ),
        );

        map
    };
}

impl SceneDescription {
    pub fn register_variable(&mut self, name: String, object: SceneObjectOrIdentifier) {
        match object {
            SceneObjectOrIdentifier::SceneObject(obj) => {
                self.variables.insert(name, obj);
            }
            SceneObjectOrIdentifier::Identifier(id) => {
                if let Some(obj) = self.get_variable(&id).cloned() {
                    self.variables.insert(name, obj.clone());
                } else {
                    panic!("UNKNOWN identifier: {}", id);
                }
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&SceneObject> {
        self.variables.get(name)
    }

    pub fn build(self) -> Vec<Box<dyn Intersectable>> {
        self.declarations
    }

    pub fn extract_params(
        &self,
        params: HashMap<String, SceneObjectOrIdentifier>,
    ) -> HashMap<String, SceneObject> {
        params
            .into_iter()
            .map(|(name, param)| {
                let obj = match param {
                    SceneObjectOrIdentifier::SceneObject(obj) => obj,
                    SceneObjectOrIdentifier::Identifier(id) => {
                        self.get_variable(&id).unwrap().clone()
                    }
                };
                (name, obj)
            })
            .collect()
    }

    pub fn declare(&mut self, creator: String, params: HashMap<String, SceneObjectOrIdentifier>) {
        let params = self.extract_params(params);
        if let Some((proto, creator)) = DECLARABLES.get(&creator) {
            typecheck_params(proto, &params);
            creator(self, params);
        } else {
            panic!("NO CREATOR: {}", creator);
        }
    }

    pub fn build_object(
        &self,
        creator: String,
        params: HashMap<String, SceneObjectOrIdentifier>,
    ) -> SceneObject {
        let params = self.extract_params(params);
        if let Some((proto, creator)) = BUILDABLES.get(&creator) {
            typecheck_params(proto, &params);
            creator(params)
        } else {
            panic!("NO CREATOR: {}", creator);
        }
    }
}

fn typecheck_params(
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
