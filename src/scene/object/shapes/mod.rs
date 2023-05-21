pub mod sphere;

use self::sphere::Sphere;
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};

pub struct ShapeHit {
    pub t: f64,
}

pub trait Shape {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<ShapeHit>;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Shapes {
    Sphere(Sphere),
}

impl Shape for Shapes {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<ShapeHit> {
        match self {
            Shapes::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}
