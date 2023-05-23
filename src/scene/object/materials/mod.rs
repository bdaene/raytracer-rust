pub mod texture;
pub mod uniform;

use crate::utils::hit::Hit;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};
use texture::Texture;
use uniform::Uniform;

pub trait Material {
    fn get_color(&self, hit: Hit) -> LinSrgb;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Materials {
    Uniform(Uniform),
    Texture(Texture),
}

impl Material for Materials {
    fn get_color(&self, hit: Hit) -> LinSrgb {
        match self {
            Materials::Uniform(uniform) => uniform.get_color(hit),
            Materials::Texture(texture) => texture.get_color(hit),
        }
    }
}
