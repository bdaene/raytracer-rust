pub mod half_space;
pub mod sky;
pub mod sphere;

use crate::utils::hit::ShapeHit;
use crate::utils::ray::Ray;
use half_space::HalfSpace;
use serde::{Deserialize, Serialize};
use sky::Sky;
use sphere::Sphere;

pub const TOLERANCE: f64 = 1e-6;

pub trait Shape {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64>;

    fn get_hit_info(&self, ray: Ray, t: f64) -> ShapeHit;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Shapes {
    Sphere(Sphere),
    Sky(Sky),
    HalfSpace(HalfSpace),
}

impl Shape for Shapes {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64> {
        match self {
            Shapes::Sphere(shape) => shape.hit(ray, t_min, t_max),
            Shapes::Sky(shape) => shape.hit(ray, t_min, t_max),
            Shapes::HalfSpace(shape) => shape.hit(ray, t_min, t_max),
        }
    }

    fn get_hit_info(&self, ray: Ray, t: f64) -> ShapeHit {
        match self {
            Shapes::Sphere(shape) => shape.get_hit_info(ray, t),
            Shapes::Sky(shape) => shape.get_hit_info(ray, t),
            Shapes::HalfSpace(shape) => shape.get_hit_info(ray, t),
        }
    }
}
