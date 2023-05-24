use super::Color;
use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use image::io::Reader;
use image::{DynamicImage, RgbImage};
use palette::{LinSrgb, Srgb};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

serde_with::serde_conv!(
    ImageAsPath,
    RgbImage,
    |_image: &RgbImage| { "".to_owned() },
    |path: String| -> Result<_, std::convert::Infallible> {
        let image = match Reader::open(path).unwrap().decode().unwrap() {
            DynamicImage::ImageRgb8(image) => image,
            _ => panic!("Invalid format"),
        };
        Ok(image)
    }
);

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Texture {
    #[serde_as(as = "ImageAsPath")]
    pub texture: RgbImage,
}

impl Color for Texture {
    fn get_color(&self, _ray: Ray, _t: f64, shape_hit: &ShapeHit) -> LinSrgb {
        let (width, height) = self.texture.dimensions();
        let (u, v) = (shape_hit.u.fract(), shape_hit.v.fract());
        let color = self
            .texture
            .get_pixel((u * width as f64) as u32, (v * height as f64) as u32)
            .0;
        Srgb::new(color[0], color[1], color[2]).into_linear()
    }
}
