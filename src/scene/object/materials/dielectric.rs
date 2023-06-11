use super::Material;
use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use palette::LinSrgb;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Material for Dielectric {
    fn scatter_ray(&self, ray: Ray, _t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        let direction = ray.direction.normalized();

        let c = -(direction.dot(&shape_hit.normal));
        let (c, r) = if c > 0. {
            (c, 1. / self.refractive_index)
        } else {
            (-c, self.refractive_index)
        };

        let delta = 1. - r * r * (1. - c * c);
        let new_direction = if delta.is_sign_negative() || self.is_reflected(c, r) {
            direction + (2. * c) * shape_hit.normal
        } else {
            r * direction + (r * c - delta.sqrt()) * shape_hit.normal
        };

        (
            LinSrgb::new(1., 1., 1.),
            Some(Ray {
                origin: shape_hit.position,
                direction: new_direction,
            }),
        )
    }
}

impl Dielectric {
    fn is_reflected(&self, cos_theta: f64, refractive_ratio: f64) -> bool {
        let r0 = (1. - refractive_ratio) / (1. + refractive_ratio);
        let r0 = r0 * r0;
        let r = r0 + (1. - r0) * (1. - cos_theta).powi(5);
        random::<f64>() < r
    }
}
