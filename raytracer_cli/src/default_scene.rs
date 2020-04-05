use std::sync::Arc;

use maplit::hashmap;
use rand;
use rand::prelude::*;

use raytracer::hittable::{self, Plane, Sphere};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::texture::{CheckerTexture, PerlinTexture, SolidTexture};
use raytracer::{self, FloatTy, Hittable, Vec3};

use crate::scene_description::SceneDescription;
use crate::PresetConfig;

pub fn default_scene_builder() -> SceneDescription {
    let default_preset = PresetConfig {
        width: 600,
        height: 400,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 20.0,
        sample_count: 1,
        max_depth: 3,
        background: Some(Vec3::all(0.1)),
    };

    let test_preset = PresetConfig {
        width: 900,
        height: 600,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 20.0,
        sample_count: 12,
        max_depth: 3,
        background: Some(Vec3::all(0.1)),
    };

    let complete_preset = PresetConfig {
        width: 1200,
        height: 800,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 20.0,
        sample_count: 128,
        max_depth: 10,
        background: Some(Vec3::all(0.1)),
    };

    let checker_texture = Arc::new(CheckerTexture::new(
        Arc::new(SolidTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Arc::new(SolidTexture::new(Vec3::all(0.9))),
        10.0,
    ));

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(Plane::with_uv(
        Vec3::all(0.0),
        Vec3::new(0.0, 1.0, 0.0),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Arc::new(Lambertian::new(checker_texture)),
    ))];

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as FloatTy + 0.9 * rng.gen::<FloatTy>(),
                0.2,
                b as FloatTy + 0.9 * rng.gen::<FloatTy>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat: FloatTy = rng.gen();

                if mat < 0.8 {
                    let albedo = Vec3::memberwise_product(rng.gen(), rng.gen());
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::from_solid_color(albedo)),
                    )));
                } else if mat < 0.95 {
                    let albedo = Vec3::new(
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                        rng.gen_range(0.5, 1.0),
                    );
                    let fuzz = rng.gen_range(0.0, 0.5);
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, Some(fuzz))),
                    )))
                } else {
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::from_solid_color(Vec3::new(0.4, 0.2, 0.1))),
    )));

    objects.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), None)),
    )));

    let declarations = hittable::build_bvh(objects);
    // let declarations = objects;

    SceneDescription {
        presets: hashmap! {
            "default".into() => default_preset,
            "complete".into() => complete_preset,
            "test".into() => test_preset
        },
        declarations,
    }
}

pub fn two_spheres() -> SceneDescription {
    let default_preset = PresetConfig {
        width: 600,
        height: 400,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 20.0,
        sample_count: 1,
        max_depth: 3,
        background: Some(Vec3::all(0.5)),
    };

    let test_preset = PresetConfig {
        width: 900,
        height: 600,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 20.0,
        sample_count: 12,
        max_depth: 3,
        background: Some(Vec3::all(0.5)),
    };

    let complete_preset = PresetConfig {
        width: 1200,
        height: 800,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 20.0,
        sample_count: 128,
        max_depth: 10,
        background: Some(Vec3::all(0.5)),
    };

    /*let ball_texture = Arc::new(CheckerTexture::new(
        Arc::new(SolidTexture::new(Vec3::new(0.6, 0.3, 0.2))),
        Arc::new(SolidTexture::new(Vec3::all(0.9))),
        20.0,
    ));*/
    let ball_texture = Arc::new(PerlinTexture::new(10.0));

    let ground_texture = Arc::new(CheckerTexture::new(
        Arc::new(SolidTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Arc::new(SolidTexture::new(Vec3::all(0.9))),
        10.0,
    ));

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(Plane::with_uv(
        Vec3::all(0.0),
        Vec3::new(0.0, 1.0, 0.0),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Arc::new(Lambertian::new(ground_texture)),
    ))];

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(ball_texture)),
    )));

    let declarations = objects;

    SceneDescription {
        presets: hashmap! {
            "default".into() => default_preset,
            "complete".into() => complete_preset,
            "test".into() => test_preset
        },
        declarations,
    }
}
