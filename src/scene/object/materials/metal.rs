use super::Material;
use crate::scene::object::colors::Color;
use crate::utils::random::get_random_on_sphere;
use crate::utils::ray::Ray;
use crate::{scene::object::colors::Colors, utils::hit::ShapeHit};
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metal {
    pub color: Colors,
    pub roughness: f64,
}

impl Material for Metal {
    fn bounce_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        let mut direction = ray.direction;
        let color = self.color.get_color(ray, t, shape_hit);
        let mut total_color = LinSrgb::new(1., 1., 1.);
        while shape_hit.normal.dot(&direction) <= 0. {
            let normal = shape_hit.normal + self.roughness * get_random_on_sphere();
            let n = normal.dot(&direction) / normal.norm_squared() * normal;
            direction = direction - 2. * n;
            total_color *= color;
        }

        (
            total_color,
            Some(Ray {
                origin: shape_hit.position,
                direction,
            }),
        )
    }
}
