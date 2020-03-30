use raytracer::{Color, FloatTy, Ray, Vec3};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn color(ray: Ray) -> Vec3 {
    let unit_dir = ray.direction.to_unit();
    let t = (unit_dir.y + 1.0) * 0.5;
    Vec3::all(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
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

fn main() {
    let nx = 200;
    let ny = 100;

    let mut image = Image::new(nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    for j in 0..ny {
        for i in 0..nx {
            let u = i as FloatTy / nx as FloatTy;
            let v = j as FloatTy / ny as FloatTy;

            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color_vec = color(ray);

            let color = Color::rgb(
                (color_vec.x * 255.0) as u8,
                (color_vec.y * 255.0) as u8,
                (color_vec.z * 255.0) as u8,
            );
            image.set_pixel(i, ny - j - 1, color);
        }
    }
    let output_path = "./img.ppm";
    image.output_as_ppm(output_path).unwrap();
}
