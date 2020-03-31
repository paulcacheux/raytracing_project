use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;

use png;
use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{Camera, Color, FloatTy, Intersectable, Ray, Sphere, Vec3};

fn color(objects: &[Box<dyn Intersectable>], ray: Ray) -> Vec3 {
    if let Some(record) = objects.is_intersected_by(&ray, 0.0, None) {
        let point = (ray.point_at_parameter(record.t) - Vec3::new(0.0, 0.0, -1.0)).to_unit();
        (point + Vec3::all(1.0)) * 0.5
    } else {
        let unit_dir = ray.direction.to_unit();
        let t = (unit_dir.y + 1.0) * 0.5;
        Vec3::all(1.0) * t + Vec3::new(0.5, 0.7, 1.0) * (1.0 - t)
    }
}

struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![Color::rgb(0, 0, 0); width * height],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.data[y * self.width + x] = color;
    }

    fn output_as_png<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        let mut stream = writer.stream_writer();

        for pixel in &self.data {
            stream.write(&[pixel.r, pixel.g, pixel.b])?;
        }

        Ok(())
    }
}

fn compute_pixel(
    camera: &Camera,
    objects: &[Box<dyn Intersectable>],
    u: FloatTy,
    v: FloatTy,
) -> Color {
    let ray = camera.get_ray(u, v);
    let color_vec = color(&objects, ray);
    Color::from_vec3(color_vec)
}

fn main() {
    let nx: usize = 800;
    let ny: usize = 600;
    let sample_count = 4;

    let aspect_ratio = (nx as FloatTy) / (ny as FloatTy);

    let mut image = Image::new(nx, ny);

    let objects: Arc<Vec<Box<dyn Intersectable>>> = Arc::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -2.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -2.0), 100.0)),
    ]);

    let camera = Arc::new(Camera::new(aspect_ratio));
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

    let output_path = "./img.png";
    image.output_as_png(output_path).unwrap();
}
