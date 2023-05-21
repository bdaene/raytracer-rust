use super::ShapeHit;
use crate::utils::point::Point;
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};

use super::Shape;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    pub position: Point,
    pub radius: f64,
}

impl Shape for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<ShapeHit> {
        // ray
        let center_origin = self.position - ray.origin;
        let direction = ray.direction;

        let a = direction.norm_squared();
        let half_b = direction.dot(&center_origin);
        let c = center_origin.norm_squared() - self.radius.powi(2);

        let quater_delta = half_b.powi(2) - a * c;
        if quater_delta >= 0. {
            for &t in [
                (half_b - quater_delta.sqrt()) / a,
                (half_b + quater_delta.sqrt()) / a,
            ]
            .iter()
            {
                if t_min < t && t < t_max {
                    return Some(ShapeHit { t });
                }
            }
        }
        None
    }
}
