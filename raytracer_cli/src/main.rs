use std::sync::mpsc;
use std::sync::Arc;

use rand;
use rand::prelude::*;
use threadpool::ThreadPool;

use raytracer::{
    Camera, Color, FloatTy, Intersectable, Lambertian, Light, Metal, Ray, Sphere, Vec3, Plan
};

mod image;

use image::Image;

fn color(objects: &[Box<dyn Intersectable>], ray: Ray, depth: usize) -> Vec3 {
    if let Some(record) = objects.is_intersected_by(&ray, 0.01, None) {
        let emitted = record.material.emit(record.u, record.v, record.p);

        if depth < 10 {
            if let Some(material_scatter) = record.material.scatter(&ray, &record) {
                return if let Some(scattered) = material_scatter.scattered {
                    let cos_theta = Vec3::dot(scattered.direction, record.normal);
                    let brdf = material_scatter.attenuation / (std::f64::consts::PI as FloatTy);
                    let p = 1.0 / (2.0 * std::f64::consts::PI as FloatTy);
                    let scattered_color = color(objects, scattered, depth + 1);
                    emitted + Vec3::memberwise_product(scattered_color, brdf)
                        * cos_theta
                        / p
                } else {
                    emitted
                };
            }
        }
        emitted
    } else {
        Vec3::all(0.0)
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
    Color::from_vec3(color_vec)
}

fn main() {
    let nx: usize = 800;
    let ny: usize = 600;
    let sample_count = 128;

    let aspect_ratio = (nx as FloatTy) / (ny as FloatTy);

    let mut image = Image::new(nx, ny);

    let objects: Arc<Vec<Box<dyn Intersectable>>> = Arc::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, -4.0),
            0.5,
            // Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
            Arc::new(Light::new(Vec3::new(1.0, 0.3, 0.3)))
        )),
        Box::new(Sphere::new(
            Vec3::new(1.0, 0.0, -4.0),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), Some(0.2))),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -3.5),
            0.5,
            Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), None)),
        )),
        /*Box::new(Sphere::new(
            Vec3::new(0.0, 3.0, 101.0),
            100.0,
            Arc::new(Matte::new(Vec3::new(1.0, 1.0, 1.0))),
        )),*/
        Box::new(Sphere::new(
            Vec3::new(12.0, 12.0, -2.0),
            3.0,
            Arc::new(Light::white()),
        )),
        Box::new(Sphere::new(
            Vec3::new(-5.0, 2.0, 2.0),
            3.0,
            Arc::new(Light::white()),
        )),
        /*Box::new(Sphere::new(
            Vec3::new(0.0, -1000.5, -2.0),
            1000.0,
            Arc::new(Lambertian::new(Vec3::new(0.47, 0.87, 0.56))),
        )),*/
        Box::new(Plan::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))),
        Box::new(Plan::new(Vec3::new(0.0, 0.0, -7.0), Vec3::new(0.0, 0.0, 1.0), Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73))))),
        Box::new(Plan::new(Vec3::new(-2.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15))))),
        Box::new(Plan::new(Vec3::new(2.5, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05))))),
    ]);

    let camera = Arc::new(Camera::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 1.0, 0.0), 90.0, aspect_ratio));
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
