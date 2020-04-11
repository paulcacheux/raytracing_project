use std::fs;
use std::io;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::Arc;

use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use lalrpop_util::lalrpop_mod;
use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{self, Camera, Color, FloatTy, Hittable, Pt3, Vec3};

mod default_scene;
mod pixel_data;
mod scene_description;
lalrpop_mod!(pub grammar);

use pixel_data::PixelData;
use scene_description::{SceneDescription, SceneDescriptionBuilder};

#[derive(Debug)]
pub struct PresetConfig {
    width: usize,
    height: usize,
    look_from: Pt3,
    look_at: Pt3,
    up: Vec3,
    vfov: FloatTy,
    sample_count: usize,
    background: Option<Vec3>,
}

fn compute_pixel<R: Rng>(
    camera: &Camera,
    objects: &[Box<dyn Hittable>],
    u: FloatTy,
    v: FloatTy,
    background: Vec3,
    rng: &mut R,
) -> Color {
    let ray = camera.get_ray(u, v);
    let color_vec = raytracer::compute_color(&objects, ray, background, rng);
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

fn search_scene(name: &str) -> SceneDescription {
    match name {
        "random_balls" => default_scene::default_scene_builder(),
        "two_spheres" => default_scene::two_spheres(),
        "cornell" => default_scene::cornell_box(),
        other => parse_input_file(other).unwrap(),
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
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("preset")
                .help("Sets the preset to run. Defaults to \"default\".")
                .short("p")
                .long("preset")
                .value_name("PRESET")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("job_count")
                .help("Set the number of jobs (threads) to run on")
                .short("j")
                .long("job")
                .default_value("4")
                .validator(validate_integer),
        )
        .get_matches();

    let scene = search_scene(matches.value_of("INPUT").unwrap());

    let objects = Arc::new(scene.declarations);
    let preset_name = matches.value_of("preset").unwrap_or("default");
    let preset = scene.presets.get(preset_name).unwrap();
    let job_count = u32::from_str(matches.value_of("job_count").unwrap()).unwrap();

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

    let background_color = preset.background.unwrap_or(Vec3::repeat(0.0));

    let sample_count = preset.sample_count;

    let mut image = PixelData::new(nx, ny);

    let (send, recv) = mpsc::channel();
    let pool = ThreadPool::new(job_count as usize);

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
                    let color = compute_pixel(&camera, &objects, u, v, background_color, &mut rng);
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
    image.save(output_path).unwrap();
}

fn validate_integer(input_value: String) -> Result<(), String> {
    u32::from_str(&input_value)
        .map(|_| ())
        .map_err(|err| err.to_string())
}
