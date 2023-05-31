pub mod diffuse;
pub mod light;
pub mod metal;

use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use diffuse::Diffuse;
use light::Light;
use metal::Metal;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

pub trait Material {
    fn bounce_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>);
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Materials {
    Metal(Metal),
    Light(Light),
    Diffuse(Diffuse),
}

impl Material for Materials {
    fn bounce_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        match self {
            Materials::Metal(material) => material.bounce_ray(ray, t, shape_hit),
            Materials::Light(material) => material.bounce_ray(ray, t, shape_hit),
            Materials::Diffuse(material) => material.bounce_ray(ray, t, shape_hit),
        }
    }
}
