use std::error::Error;
use std::sync::Arc;

use lazy_static::lazy_static;
use maplit::hashmap;
use raytracer::hittable::{self, TriangleBuilder};
use std::collections::HashMap;

use raytracer::material::Lambertian;
use raytracer::texture::{ImageTexture, SolidTexture};
use raytracer::{FloatTy, Pt3, Vec3};
use tobj;

use crate::{PresetConfig, SceneDescription};

lazy_static! {
    static ref RED_MAT: Arc<Lambertian<SolidTexture>> =
        Arc::new(Lambertian::from_solid_color(Vec3::new(0.9, 0.3, 0.3)));
    static ref TEXTURE_MAT: Arc<Lambertian<ImageTexture>> = {
        let texture = ImageTexture::open(
            "/home/paul/Downloads/cartoon-house/textures/CoatedCartoonHouse_None_color.jpg",
        )
        .unwrap();
        Arc::new(Lambertian::new(texture))
    };
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
        let has_texcoords = !mesh.texcoords.is_empty();

        assert_eq!(mesh.indices.len() % 3, 0);

        for indices in mesh.indices.chunks(3) {
            let mut points = Vec::with_capacity(3);
            let mut normals = Vec::with_capacity(3);
            let mut texcoords = Vec::with_capacity(3);

            for index in indices {
                let index = *index as usize;
                let pos = Pt3::new(
                    mesh.positions[index * 3] as FloatTy,
                    mesh.positions[index * 3 + 1] as FloatTy,
                    mesh.positions[index * 3 + 2] as FloatTy,
                );

                points.push(pos);

                if has_normals {
                    let normal = Vec3::new(
                        mesh.normals[index * 3] as FloatTy,
                        mesh.normals[index * 3 + 1] as FloatTy,
                        mesh.normals[index * 3 + 2] as FloatTy,
                    );
                    normals.push(normal);
                }

                if has_texcoords {
                    let coords = [
                        mesh.texcoords[index * 2] as FloatTy,
                        mesh.texcoords[index * 2 + 1] as FloatTy,
                    ];
                    texcoords.push(coords);
                }
            }

            let mut builder =
                TriangleBuilder::new([points[0], points[1], points[2]], TEXTURE_MAT.clone());

            if has_normals {
                builder = builder.with_normals([normals[0], normals[1], normals[2]]);
            }

            if has_texcoords {
                builder = builder.with_texcoords([texcoords[0], texcoords[1], texcoords[2]]);
            }

            let triangle = builder.build();
            objects.push(Box::new(triangle));
        }
    }

    let declarations = hittable::build_bvh(objects);

    Ok(SceneDescription {
        presets: presets_for_obj(path),
        declarations,
    })
}
