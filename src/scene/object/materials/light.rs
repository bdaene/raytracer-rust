use super::Material;
use crate::scene::object::colors::Color;
use crate::{utils::hit::ShapeHit, scene::object::colors::Colors};
use crate::utils::ray::Ray;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Light {
    pub color: Colors,
}

impl Material for Light {
    fn bounce_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        (self.color.get_color(ray, t, shape_hit), None)
    }
}
