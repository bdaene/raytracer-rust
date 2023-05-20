pub mod sphere;

use crate::utils::ray::Ray;

pub struct ShapeHit {
    pub t: f64,
}

pub trait Shape {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<ShapeHit>;
}
