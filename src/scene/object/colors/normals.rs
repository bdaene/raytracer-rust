use super::Color;
use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Normals {}

impl Color for Normals {
    fn get_color(&self, _ray: Ray, _t: f64, shape_hit: &ShapeHit) -> LinSrgb {
        LinSrgb::new(
            0.5 + (shape_hit.normal.x() as f32) / 2.,
            0.5 + (shape_hit.normal.y() as f32) / 2.,
            0.5 + (shape_hit.normal.z() as f32) / 2.,
        )
    }
}
