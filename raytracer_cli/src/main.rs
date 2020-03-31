use std::sync::mpsc;
use std::sync::Arc;

use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{Camera, Color, FloatTy, Intersectable, Lambertian, Metal, Ray, Sphere, Vec3};

mod image;

use image::Image;

fn color(objects: &[Box<dyn Intersectable>], ray: Ray, depth: usize) -> Vec3 {
    if let Some(record) = objects.is_intersected_by(&ray, 0.001, None) {
        if depth < 50 {
            if let Some(material_scatter) = record.material.scatter(&ray, &record) {
                return Vec3::memberwise_product(
                    color(objects, material_scatter.scattered, depth + 1),
                    material_scatter.attenuation,
                );
            }
        }
        Vec3::all(0.0)
    } else {
        let unit_dir = ray.direction.to_unit();
        let t = (unit_dir.y + 1.0) * 0.5;
        Vec3::all(1.0) * t + Vec3::new(0.5, 0.7, 1.0) * (1.0 - t)
    }
}

fn compute_pixel(
    camera: &Camera,
    objects: &[Box<dyn Intersectable>],
    u: FloatTy,
    v: FloatTy,
) -> Color {
    let ray = camera.get_ray(u, v);
    let color_vec = color(&objects, ray, 0);
    Color::from_vec3_gamma_corrected(color_vec)
}

fn main() {
    let nx: usize = 1280;
    let ny: usize = 720;
    let sample_count = 8;

    let aspect_ratio = (nx as FloatTy) / (ny as FloatTy);

    let mut image = Image::new(nx, ny);

    let objects: Arc<Vec<Box<dyn Intersectable>>> = Arc::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -3.0),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -3.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), Some(0.2))),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -3.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), None)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -2.0),
            100.0,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
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

    let output_path = "./last_result.png";
    image.output_as_png(output_path).unwrap();
}
