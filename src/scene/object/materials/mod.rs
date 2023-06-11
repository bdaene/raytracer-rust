pub mod dielectric;
pub mod diffuse;
pub mod light;
pub mod metal;

use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use dielectric::Dielectric;
use diffuse::Diffuse;
use light::Light;
use metal::Metal;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};

pub trait Material {
    fn scatter_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>);
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Materials {
    Metal(Metal),
    Light(Light),
    Diffuse(Diffuse),
    Dielectric(Dielectric),
}

impl Material for Materials {
    fn scatter_ray(&self, ray: Ray, t: f64, shape_hit: &ShapeHit) -> (LinSrgb, Option<Ray>) {
        match self {
            Materials::Metal(material) => material.scatter_ray(ray, t, shape_hit),
            Materials::Light(material) => material.scatter_ray(ray, t, shape_hit),
            Materials::Diffuse(material) => material.scatter_ray(ray, t, shape_hit),
            Materials::Dielectric(material) => material.scatter_ray(ray, t, shape_hit),
        }
    }
}
