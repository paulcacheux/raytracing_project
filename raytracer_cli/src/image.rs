use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use png;

use raytracer::Color;

#[derive(Debug, Clone)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            data: vec![Color::rgb(0, 0, 0); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.data[y * self.width + x] = color;
    }

    pub fn output_as_png<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
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
