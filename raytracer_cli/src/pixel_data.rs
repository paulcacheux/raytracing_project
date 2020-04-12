use std::path::Path;

use image::RgbaImage;

use raytracer::{Color, FloatTy, Vec3};

#[derive(Debug, Clone)]
pub struct PixelData {
    width: usize,
    height: usize,
    buffer: Vec<(Vec3, usize)>,
}

impl PixelData {
    pub fn new(width: usize, height: usize) -> Self {
        PixelData {
            width,
            height,
            buffer: vec![(Vec3::zeros(), 0); width * height],
        }
    }

    pub fn append_pixel(&mut self, x: usize, y: usize, color: Vec3) {
        if color.x.is_nan() || color.y.is_nan() || color.z.is_nan() {
            return;
        }

        let (current_color, count) = self.buffer[(y * self.width + x) as usize];
        let next_color = current_color + color;
        self.buffer[(y * self.width + x)] = (next_color, count + 1);
    }

    pub fn as_image(&self) -> RgbaImage {
        RgbaImage::from_fn(self.width as _, self.height as _, |x, y| {
            let x = x as usize;
            let y = y as usize;
            let (sum_color, count) = self.buffer[y * self.width + x];
            let color = sum_color / (count as FloatTy);
            let color = Color::from_vec3(color).to_rgb();
            image::Rgba([color[0], color[1], color[2], 255])
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), image::error::ImageError> {
        self.as_image().save(path)
    }
}
