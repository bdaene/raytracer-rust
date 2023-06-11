use super::Material;
use crate::scene::object::colors::{Color, Colors};
use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub color: Colors,
    pub power: f32,
}

impl Material for Light {
    fn scatter_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        let color = self.color.get_color(ray, t, shape_hit) * self.power;
        (color, None)
    }
}
