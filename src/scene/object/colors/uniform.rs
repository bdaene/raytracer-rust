use super::Color;
use crate::scene::LinSrgbAsArray;
use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Uniform {
    #[serde_as(as = "LinSrgbAsArray")]
    pub color: LinSrgb,
}

impl Color for Uniform {
    fn get_color(&self, _ray: Ray, _t: f64, _shape_hit: &ShapeHit) -> LinSrgb {
        self.color
    }
}
