use super::{Shape, ShapeHit, TOLERANCE};
use crate::utils::point::{Point, PointAsArray};
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sphere {
    #[serde_as(as = "PointAsArray")]
    pub position: Point,
    pub radius: f64,
}

impl Shape for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let center_origin = self.position - ray.origin;
        let direction = ray.direction;

        let a = direction.norm_squared();
        let half_b = direction.dot(&center_origin);
        let c = center_origin.norm_squared() - self.radius.powi(2);

        let quater_delta = half_b.powi(2) - a * c;
        if quater_delta <= TOLERANCE {
            return None;
        }

        let m = half_b / a;
        let d = quater_delta.sqrt() / a;
        [m - d, m + d]
            .iter()
            .copied()
            .filter(|&t| t_min < t && t < t_max)
            .next()
    }

    fn get_hit_info(&self, ray: Ray, t: f64) -> ShapeHit {
        let position = ray.at_t(t);
        let normal = (position - self.position).normalized();
        let u = 0.5 + normal.y().atan2(normal.x()) / TAU;
        let v = 0.5 - normal.z().asin() / PI;

        ShapeHit {
            position,
            normal,
            u,
            v,
        }
    }
}
