use std::fs;
use std::io;
use std::sync::mpsc;
use std::sync::Arc;

use lalrpop_util::lalrpop_mod;
use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{self, Camera, Color, FloatTy, Intersectable, Vec3};

mod image;
mod scene_description;
lalrpop_mod!(pub grammar);

use image::Image;
use scene_description::SceneDescription;

fn compute_pixel(
    camera: &Camera,
    objects: &[Box<dyn Intersectable>],
    u: FloatTy,
    v: FloatTy,
) -> Color {
    let ray = camera.get_ray(u, v);
    let color_vec = raytracer::compute_color(&objects, ray, 0);
    Color::from_vec3(color_vec)
}

fn parse_input_file(path: &str) -> io::Result<Vec<Box<dyn Intersectable>>> {
    let content = fs::read_to_string(path)?;

    let mut scene_description = SceneDescription::default();
    let parser = grammar::ProgramParser::new();
    parser.parse(&mut scene_description, &content).unwrap();

    let world = scene_description.build();
    Ok(world)
}

fn main() {
    let nx: usize = 800;
    let ny: usize = 600;
    let sample_count = 1;

    let aspect_ratio = (nx as FloatTy) / (ny as FloatTy);

    let mut image = Image::new(nx, ny);

    let input_path = std::env::args().nth(1).unwrap();
    let world = parse_input_file(&input_path).unwrap();

    let objects = Arc::new(world);

    let camera = Arc::new(Camera::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        aspect_ratio,
    ));
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
                    let color = compute_pixel(&camera, &objects, u, v);
                    colors.push(color);
                }
                local_send.send((i, j, Color::average(&colors))).unwrap();
            }
        })
    }
    drop(send);

    for (i, j, color) in recv.into_iter() {
        image.set_pixel(i, j, color);
    }

    let output_path = "./last_result.png";
    image.output_as_png(output_path).unwrap();
}
