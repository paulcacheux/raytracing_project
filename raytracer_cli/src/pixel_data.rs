use std::path::Path;

use image::RgbImage;

use raytracer::Color;

const GAMMA_CORRECTION: bool = true;

#[derive(Debug, Clone)]
pub struct PixelData {
    width: usize,
    height: usize,
    buffer: RgbImage,
}

impl PixelData {
    pub fn new(width: usize, height: usize) -> Self {
        PixelData {
            width,
            height,
            buffer: RgbImage::new(width as _, height as _),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let color = if GAMMA_CORRECTION {
            color.gamma_corrected()
        } else {
            color
        };
        self.buffer
            .put_pixel(x as _, y as _, image::Rgb([color.r, color.g, color.b]));
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), image::error::ImageError> {
        self.buffer.save(path)
    }
}
