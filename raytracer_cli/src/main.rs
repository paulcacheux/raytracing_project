use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use clap::{App, Arg};
use env_logger;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::{iproduct, Itertools};
use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{self, Camera, FloatTy, Hittable, Pt3, Vec3};

mod default_scene;
mod gui;
mod obj;
mod pixel_data;

use pixel_data::PixelData;

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

pub struct SceneDescription {
    pub declarations: Vec<Box<dyn Hittable>>,
    pub presets: HashMap<String, PresetConfig>,
}

const THREAD_CHUNK_SIZE: usize = 20000;

fn compute_pixel<R: Rng>(
    camera: &Camera,
    objects: &[Box<dyn Hittable>],
    u: FloatTy,
    v: FloatTy,
    background: Vec3,
    rng: &mut R,
) -> Vec3 {
    let ray = camera.get_ray(u, v);
    let color_vec = raytracer::compute_color(&objects, ray, background, rng);
    color_vec
}

fn search_scene(name: &str) -> SceneDescription {
    match name {
        "random_balls" => default_scene::default_scene_builder(),
        "two_spheres" => default_scene::two_spheres(),
        "cornell" => default_scene::cornell_box(),
        other => obj::load_obj(other).unwrap(),
    }
}

fn main() {
    env_logger::init();

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

    let image = Arc::new(Mutex::new(PixelData::new(nx, ny)));

    let (send, recv) = mpsc::channel();
    let pool = ThreadPool::new(job_count as usize);

    let main_iter = iproduct!(0..sample_count, 0..ny, 0..nx);

    for chunks in &main_iter.chunks(THREAD_CHUNK_SIZE) {
        let local_send = send.clone();
        let camera = camera.clone();
        let objects = objects.clone();
        let chunks: Vec<_> = chunks.collect();

        pool.execute(move || {
            for (_, y, x) in chunks {
                let mut rng = rand::thread_rng();

                let di: FloatTy = rng.gen();
                let dj: FloatTy = rng.gen();

                let u = (x as FloatTy + di) / nx as FloatTy;
                let v = ((ny - y - 1) as FloatTy + dj) / ny as FloatTy;
                let color = compute_pixel(&camera, &objects, u, v, background_color, &mut rng);
                local_send.send((x, y, color)).unwrap();
            }
        })
    }

    drop(send);

    let progress_bar = ProgressBar::new((ny * nx * sample_count) as _);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );

    let rec_image = image.clone();
    let recuperator = thread::spawn(move || {
        for (x, y, color) in recv.into_iter() {
            rec_image.lock().unwrap().append_pixel(x, y, color);
            progress_bar.inc(1);
        }
    });

    gui::run::<gui::RayTracingGUI>(nx, ny, image);

    recuperator.join().unwrap();

    /*
    let output_path = "./last_result.png";
    image.lock().unwrap().save(output_path).unwrap();
    */
}

fn validate_integer(input_value: String) -> Result<(), String> {
    u32::from_str(&input_value)
        .map(|_| ())
        .map_err(|err| err.to_string())
}
