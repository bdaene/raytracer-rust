use super::Material;
use crate::scene::object::colors::Color;
use crate::utils::random::get_random_on_sphere;
use crate::utils::ray::Ray;
use crate::{scene::object::colors::Colors, utils::hit::ShapeHit};
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Diffuse {
    pub color: Colors,
}

impl Material for Diffuse {
    fn bounce_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        let direction = shape_hit.normal + get_random_on_sphere();

        (
            self.color.get_color(ray, t, shape_hit),
            Some(Ray {
                origin: shape_hit.position,
                direction,
            }),
        )
    }
}
