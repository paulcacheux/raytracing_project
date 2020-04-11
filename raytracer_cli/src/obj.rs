use std::error::Error;
use std::sync::Arc;

use lazy_static::lazy_static;
use maplit::hashmap;
use raytracer::hittable::{self, Triangle};
use std::collections::HashMap;

use raytracer::material::Lambertian;
use raytracer::texture::SolidTexture;
use raytracer::{FloatTy, Pt3, Vec3};
use tobj;

use crate::{PresetConfig, SceneDescription};

lazy_static! {
    static ref RED_MAT: Arc<Lambertian<SolidTexture>> =
        Arc::new(Lambertian::from_solid_color(Vec3::new(0.9, 0.3, 0.3)));
}

fn presets_for_obj(path: &str) -> HashMap<String, PresetConfig> {
    let default_preset = if path.contains("moto") {
        PresetConfig {
            width: 200,
            height: 200,
            look_from: Pt3::new(-10.0, 5.0, 10.0),
            look_at: Pt3::new(1.0, 4.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            vfov: 45.0,
            sample_count: 1,
            background: Some(Vec3::repeat(0.2)),
        }
    } else if path.contains("CartoonHouse") {
        PresetConfig {
            width: 200,
            height: 200,
            look_from: Pt3::new(-10.0, 0.0, 10.0),
            look_at: Pt3::new(1.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            vfov: 45.0,
            sample_count: 1,
            background: Some(Vec3::repeat(0.2)),
        }
    } else {
        unimplemented!()
    };

    let test_preset = PresetConfig {
        sample_count: 10,
        ..default_preset
    };

    let full_preset = PresetConfig {
        width: 400,
        height: 400,
        sample_count: 40,
        ..default_preset
    };

    hashmap! {
        "default".into() => default_preset,
        "test".into() => test_preset,
        "full".into() => full_preset,
    }
}

pub fn load_obj(path: &str) -> Result<SceneDescription, Box<dyn Error>> {
    let (models, _) = tobj::load_obj(path)?;

    let mut objects: Vec<Box<dyn hittable::Hittable>> = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        let has_normals = !mesh.normals.is_empty();

        assert_eq!(mesh.indices.len() % 3, 0);

        for indices in mesh.indices.chunks(3) {
            let (points, normals): (Vec<_>, Vec<_>) = indices
                .into_iter()
                .map(|&index| {
                    let index = index as usize;
                    let pos = Pt3::new(
                        mesh.positions[index * 3] as FloatTy,
                        mesh.positions[index * 3 + 1] as FloatTy,
                        mesh.positions[index * 3 + 2] as FloatTy,
                    );

                    let normal = if has_normals {
                        Some(Vec3::new(
                            mesh.normals[index * 3] as FloatTy,
                            mesh.normals[index * 3 + 1] as FloatTy,
                            mesh.normals[index * 3 + 2] as FloatTy,
                        ))
                    } else {
                        None
                    };
                    (pos, normal)
                })
                .unzip();

            let normal = if has_normals {
                let normals: Vec<_> = normals.into_iter().map(Option::unwrap).collect();
                let count = normals.len();
                let sum: Vec3 = normals.into_iter().sum();
                Some(sum / (count as FloatTy))
            } else {
                None
            };

            let triangle = Triangle::new(points[0], points[1], points[2], normal, RED_MAT.clone());
            objects.push(Box::new(triangle));
        }
    }

    let declarations = hittable::build_bvh(objects);

    Ok(SceneDescription {
        presets: presets_for_obj(path),
        declarations,
    })
}
