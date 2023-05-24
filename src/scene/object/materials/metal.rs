use super::Material;
use crate::scene::object::colors::Color;
use crate::utils::ray::Ray;
use crate::{scene::object::colors::Colors, utils::hit::ShapeHit};
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metal {
    pub color: Colors,
}

impl Material for Metal {
    fn bounce_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        let n = shape_hit.normal.dot(&ray.direction) * shape_hit.normal;
        let direction = ray.direction - 2. * n;
        (
            self.color.get_color(ray, t, shape_hit),
            Some(Ray {
                origin: shape_hit.position,
                direction,
            }),
        )
    }
}
