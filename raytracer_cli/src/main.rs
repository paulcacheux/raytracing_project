use std::fs;
use std::io;
use std::sync::mpsc;
use std::sync::Arc;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use lalrpop_util::lalrpop_mod;
use maplit::hashmap;
use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::hittable::{self, Plane, Sphere};
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::{self, Camera, Color, FloatTy, Hittable, Vec3};

mod image;
mod scene_description;
lalrpop_mod!(pub grammar);

use image::Image;
use scene_description::{SceneDescription, SceneDescriptionBuilder};

#[derive(Debug)]
pub struct PresetConfig {
    width: usize,
    height: usize,
    look_from: Vec3,
    look_at: Vec3,
    up: Vec3,
    vfov: FloatTy,
    sample_count: usize,
    max_depth: usize,
    background: Option<Vec3>,
}

fn compute_pixel(
    camera: &Camera,
    objects: &[Box<dyn Hittable>],
    u: FloatTy,
    v: FloatTy,
    max_depth: usize,
    background: Vec3,
) -> Color {
    let ray = camera.get_ray(u, v);
    let color_vec = raytracer::compute_color(&objects, ray, 0, max_depth, background);
    Color::from_vec3(color_vec)
}

fn parse_input_file(path: &str) -> io::Result<SceneDescription> {
    let content = fs::read_to_string(path)?;

    let mut scene_description = SceneDescriptionBuilder::default();
    let parser = grammar::ProgramParser::new();
    parser.parse(&mut scene_description, &content).unwrap();

    let scene = scene_description.build();
    Ok(scene)
}

fn default_scene_builder() -> SceneDescription {
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

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(Plane::new(
        Vec3::all(0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Arc::new(Lambertian::new(Vec3::all(0.5))),
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
                        Arc::new(Lambertian::new(albedo)),
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
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
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

fn main() {
    let matches = App::new("Raytracing CLI")
        .version("0.1")
        .author("Paul Cacheux <paulcacheux@gmail.com>")
        .about("Raytracing utility")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input description file.")
                .index(1),
        )
        .arg(
            Arg::with_name("preset")
                .help("Sets the preset to run. Defaults to \"default\".")
                .short("p")
                .long("preset")
                .value_name("PRESET")
                .takes_value(true),
        )
        .get_matches();

    let scene = if let Some(input_path) = matches.value_of("INPUT") {
        parse_input_file(&input_path).unwrap()
    } else {
        default_scene_builder()
    };

    let objects = Arc::new(scene.declarations);
    let preset_name = matches.value_of("preset").unwrap_or("default");
    let preset = scene.presets.get(preset_name).unwrap();

    let nx: usize = preset.width;
    let ny: usize = preset.height;
    let aspect_ratio = (nx as FloatTy) / (ny as FloatTy);

    let camera = Arc::new(Camera::new(
        preset.look_from,
        preset.look_at,
        preset.up,
        preset.vfov,
        aspect_ratio,
    ));

    let background_color = preset.background.unwrap_or(Vec3::all(0.0));
    let max_depth = preset.max_depth;

    let sample_count = preset.sample_count;

    let mut image = Image::new(nx, ny);

    let (send, recv) = mpsc::channel();
    let pool = ThreadPool::new(16);

    for j in 0..ny {
        let local_send = send.clone();
        let camera = camera.clone();
        let objects = objects.clone();

        pool.execute(move || {
            let mut rng = rand::thread_rng();

            for i in 0..nx {
                let mut colors = Vec::with_capacity(sample_count);
                for _ in 0..sample_count {
                    let di: FloatTy = rng.gen();
                    let dj: FloatTy = rng.gen();

                    let u = (i as FloatTy + di) / nx as FloatTy;
                    let v = ((ny - j - 1) as FloatTy + dj) / ny as FloatTy;
                    let color = compute_pixel(&camera, &objects, u, v, max_depth, background_color);
                    colors.push(color);
                }
                local_send.send((i, j, Color::average(&colors))).unwrap();
            }
        })
    }
    drop(send);

    let progress_bar = ProgressBar::new((ny * nx) as _);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );

    for (i, j, color) in recv.into_iter() {
        image.set_pixel(i, j, color);
        progress_bar.inc(1);
    }

    let output_path = "./last_result.png";
    image.output_as_png(output_path).unwrap();
}
