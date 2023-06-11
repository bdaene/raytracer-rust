use super::Material;
use crate::scene::object::colors::{Color, Colors};
use crate::utils::hit::ShapeHit;
use crate::utils::random::get_random_on_sphere;
use crate::utils::ray::Ray;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Diffuse {
    pub color: Colors,
}

impl Material for Diffuse {
    fn scatter_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
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
