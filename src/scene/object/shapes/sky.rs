use super::{Shape, ShapeHit};
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sky {}

impl Shape for Sky {
    fn hit(&self, _ray: Ray, _t_min: f64, t_max: f64) -> Option<f64> {
        Some(t_max)
    }

    fn get_hit_info(&self, ray: Ray, _t: f64) -> ShapeHit {
        let normal = -ray.direction.normalized();
        let u = 0.5 - normal.y().atan2(normal.x()) / TAU;
        let v = 0.5 + normal.z().asin() / PI;

        ShapeHit {
            position: ray.origin,
            normal,
            u,
            v,
        }
    }
}
