#![allow(dead_code)]

use std::sync::Arc;

use maplit::hashmap;
use rand;
use rand::prelude::*;

use raytracer::hittable::{self, Plane, Sphere, XYRect, XZRect, YZRect};
use raytracer::material::{Dielectric, Lambertian, Light, Metal};
use raytracer::texture::{CheckerTexture, ImageTexture, SolidTexture};
use raytracer::{self, FloatTy, Hittable, HittableExt, Vec3};

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
        max_depth: 5,
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

    let checker_texture = CheckerTexture::new(
        Arc::new(SolidTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Arc::new(SolidTexture::new(Vec3::all(0.9))),
        10.0,
    );

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
        look_at: Vec3::new(0.0, 2.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 60.0,
        sample_count: 1,
        max_depth: 3,
        background: None,
    };

    let test_preset = PresetConfig {
        width: 900,
        height: 600,
        sample_count: 12,
        max_depth: 3,
        ..default_preset
    };

    let complete_preset = PresetConfig {
        width: 1200,
        height: 800,
        sample_count: 128,
        max_depth: 10,
        ..default_preset
    };

    /*let ball_texture = Arc::new(CheckerTexture::new(
        Arc::new(SolidTexture::new(Vec3::new(0.6, 0.3, 0.2))),
        Arc::new(SolidTexture::new(Vec3::all(0.9))),
        20.0,
    ));*/
    // let ball_texture = Arc::new(PerlinTexture::new(10.0));

    let earth_texture = Arc::new(ImageTexture::open("./textures/earthmap.jpg").unwrap());

    let ground_texture = CheckerTexture::new(
        Arc::new(SolidTexture::new(Vec3::new(0.2, 0.3, 0.1))),
        Arc::new(SolidTexture::new(Vec3::all(0.9))),
        10.0,
    );

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(Plane::with_uv(
        Vec3::all(0.0),
        Vec3::new(0.0, 1.0, 0.0),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        Arc::new(Lambertian::new(ground_texture)),
    ))];

    objects.push(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(earth_texture)),
    )));

    objects.push(Box::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -1.5,
        Arc::new(Light::white()),
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

pub fn cornell_box() -> SceneDescription {
    let default_preset = PresetConfig {
        width: 400,
        height: 400,
        look_from: Vec3::new(278.0, 278.0, -800.0),
        look_at: Vec3::new(278.0, 278.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        vfov: 40.0,
        sample_count: 1,
        max_depth: 3,
        background: None,
    };

    let test_preset = PresetConfig {
        width: 600,
        height: 600,
        sample_count: 128,
        max_depth: 40,
        ..default_preset
    };

    let complete_preset = PresetConfig {
        width: 600,
        height: 600,
        sample_count: 300,
        max_depth: 10,
        ..default_preset
    };

    let red = Arc::new(Lambertian::from_solid_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_solid_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_solid_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(Light::new(Vec3::all(15.0)));

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(
        YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green.clone()).flip_face(),
    ));

    objects.push(Box::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.push(Box::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.push(Box::new(
        XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()).flip_face(),
    ));
    objects.push(Box::new(
        XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()).flip_face(),
    ));
    objects.push(Box::new(
        XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light).flip_face(),
    ));

    SceneDescription {
        presets: hashmap! {
            "default".into() => default_preset,
            "test".into() => test_preset,
            "complete".into() => complete_preset,
        },
        declarations: objects,
    }
}
