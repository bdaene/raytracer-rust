pub mod sky;
pub mod sphere;

use crate::utils::point::Point;
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};
use sky::Sky;
use sphere::Sphere;

pub struct ShapeHit {
    pub t: f64,
    pub position: Point,
    pub normal: Point,

    pub u: f64,
    pub v: f64,
}

pub trait Shape {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<ShapeHit>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Shapes {
    Sphere(Sphere),
    Sky(Sky),
}

impl Shape for Shapes {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<ShapeHit> {
        match self {
            Shapes::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Shapes::Sky(sky) => sky.hit(ray, t_min, t_max),
        }
    }
}
