use std::fs;
use std::io;
use std::sync::mpsc;
use std::sync::Arc;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use lalrpop_util::lalrpop_mod;
use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{self, Camera, Color, FloatTy, Hittable, Vec3};

mod default_scene;
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
        default_scene::two_spheres()
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
