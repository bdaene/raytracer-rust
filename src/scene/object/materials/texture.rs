use super::Material;
use crate::utils::hit::Hit;
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

impl Material for Texture {
    fn get_color(&self, hit: Hit) -> LinSrgb {
        let (width, height) = self.texture.dimensions();
        let (u, v) = (hit.shape_hit.u.fract(), hit.shape_hit.v.fract());
        let color = self
            .texture
            .get_pixel((u * width as f64) as u32, (v * height as f64) as u32)
            .0;
        Srgb::new(color[0], color[1], color[2]).into_linear()
    }
}
