use std::path::Path;

use image::error::ImageResult;
use image::RgbImage;

use super::Texture;
use crate::{FloatTy, Vec3};

#[derive(Debug)]
pub struct ImageTexture {
    pub buffer: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    pub fn open<P: AsRef<Path>>(path: P) -> ImageResult<Self> {
        let buffer = image::open(path)?;
        let buffer = buffer.to_rgb();
        let (width, height) = buffer.dimensions();
        Ok(ImageTexture {
            buffer,
            width,
            height,
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: FloatTy, v: FloatTy) -> Vec3 {
        let u = u.fract();
        let v = 1.0 - v.fract();
        let x = (u * self.width as FloatTy) as u32;
        let y = (v * self.height as FloatTy) as u32;
        let pixel = self.buffer.get_pixel(x, y);
        let r = pixel[0] as FloatTy / 255.0;
        let g = pixel[1] as FloatTy / 255.0;
        let b = pixel[2] as FloatTy / 255.0;

        Vec3::new(r, g, b)
    }
}
