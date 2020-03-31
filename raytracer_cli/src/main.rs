use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;

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

    fn output_as_ppm<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        write!(file, "P3\n{} {}\n255\n", self.width, self.height)?;

        for pixel in &self.data {
            write!(file, "{} {} {}\n", pixel.r, pixel.g, pixel.b)?;
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
            for i in 0..nx {
                let u = i as FloatTy / nx as FloatTy;
                let v = (ny - j - 1) as FloatTy / ny as FloatTy;
                let color = compute_pixel(&camera, &objects, u, v);
                local_send.send((i, j, color)).unwrap();
            }
        })
    }
    drop(send);

    for (i, j, color) in recv.into_iter() {
        image.set_pixel(i, j, color);
    }

    let output_path = "./img.ppm";
    image.output_as_ppm(output_path).unwrap();
}
