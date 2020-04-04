use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

use super::{creators, SceneDescription, SceneObject, SceneObjectKind, SceneObjectOrIdentifier};
use crate::PresetConfig;
use raytracer::Hittable;

#[derive(Default)]
pub struct SceneDescriptionBuilder {
    variables: HashMap<String, SceneObject>,
    declarations: Vec<Box<dyn Hittable>>,
    presets: HashMap<String, PresetConfig>,
}

type DeclCreatorFn = fn(params: HashMap<String, SceneObject>) -> Box<dyn Hittable>;
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
    static ref PRESET_FIELDS: HashMap<String, (SceneObjectKind, bool)> = hashmap! {
        "width".to_string() => (SceneObjectKind::Float, true),
        "height".to_string() => (SceneObjectKind::Float, true),
        "look_from".to_string() => (SceneObjectKind::Vec3, true),
        "look_at".to_string() => (SceneObjectKind::Vec3, true),
        "up".to_string() => (SceneObjectKind::Vec3, true),
        "vfov".to_string() => (SceneObjectKind::Float, true),
        "sample_count".to_string() => (SceneObjectKind::Float, true),
        "max_depth".to_string() => (SceneObjectKind::Float, true),
        "background".to_string() => (SceneObjectKind::Vec3, false),
    };
}

impl SceneDescriptionBuilder {
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

    pub fn build(self) -> SceneDescription {
        SceneDescription {
            declarations: self.declarations,
            presets: self.presets,
        }
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
            creators::typecheck_params(proto, &params);
            let decl = creator(params);
            self.declarations.push(decl);
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
            creators::typecheck_params(proto, &params);
            creator(params)
        } else {
            panic!("NO CREATOR: {}", creator);
        }
    }

    pub fn add_preset(&mut self, name: String, params: HashMap<String, SceneObjectOrIdentifier>) {
        let params = self.extract_params(params);
        creators::typecheck_params(&PRESET_FIELDS, &params);
        let preset = creators::preset_creator(params);
        self.presets.insert(name, preset);
    }
}
