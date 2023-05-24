pub mod normals;
pub mod texture;
pub mod uniform;

use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use normals::Normals;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};
use texture::Texture;
use uniform::Uniform;

pub trait Color {
    fn get_color(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> LinSrgb;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Colors {
    Uniform(Uniform),
    Texture(Texture),
    Normals(Normals),
}

impl Color for Colors {
    #[inline]
    fn get_color(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> LinSrgb {
        match self {
            Colors::Uniform(color) => color.get_color(ray, t, shape_hit),
            Colors::Texture(color) => color.get_color(ray, t, shape_hit),
            Colors::Normals(color) => color.get_color(ray, t, shape_hit),
        }
    }
}
