use crate::utils::color::Color;
use std::fs::{create_dir_all, File};
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Pixels {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
}

const DEPTH: usize = 3;

impl Pixels {
    pub fn new(width: usize, height: usize) -> Pixels {
        Pixels {
            width,
            height,
            pixels: vec![0; width * height * DEPTH],
        }
    }

    pub fn set_color(&mut self, row: usize, col: usize, color: Color) {
        let offset = (self.width * row + col) * DEPTH;

        self.pixels[offset] = (color.red.clamp(0., 1.) * 255.) as u8;
        self.pixels[offset + 1] = (color.green.clamp(0., 1.) * 255.) as u8;
        self.pixels[offset + 2] = (color.blue.clamp(0., 1.) * 255.) as u8;
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        let mut encoder = png::Encoder::new(
            io::BufWriter::new(file),
            self.width as u32,
            self.height as u32,
        );
        encoder.set_color(png::ColorType::Rgb);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_expected_pixels() -> Pixels {
        Pixels {
            width: 4,
            height: 2,
            pixels: vec![
                0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, // First row
                255, 255, 255, 0, 255, 255, 255, 0, 255, 255, 255, 0, // Second row
            ],
        }
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Pixels::new(3, 2),
            Pixels {
                width: 3,
                height: 2,
                pixels: vec![0u8; 3 * 2 * 3]
            }
        );
    }

    #[test]
    fn test_set_color() {
        let mut pixels = Pixels::new(4, 2);

        for col in 0..4 {
            pixels.set_color(
                0,
                col,
                Color {
                    red: if col == 1 { 1. } else { 0. },
                    green: if col == 2 { 1. } else { 0. },
                    blue: if col == 3 { 1. } else { 0. },
                },
            );
            pixels.set_color(
                1,
                col,
                Color {
                    red: if col != 1 { 1. } else { 0. },
                    green: if col != 2 { 1. } else { 0. },
                    blue: if col != 3 { 1. } else { 0. },
                },
            );
        }

        assert_eq!(pixels, get_expected_pixels());
    }

    #[test]
    fn test_save() {
        let pixels = get_expected_pixels();

        pixels.save(Path::new("./tmp/pixels_save.png")).unwrap();
    }
}
