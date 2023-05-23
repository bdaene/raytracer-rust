use super::{Shape, ShapeHit};
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sky {}

impl Shape for Sky {
    fn hit(&self, ray: Ray, _t_min: f64, t_max: f64) -> Option<ShapeHit> {
        let position = ray.origin;
        let normal = -ray.direction.normalized();
        let u = 0.5 - normal.y().atan2(normal.x()) / TAU;
        let v = 0.5 + normal.z().asin() / PI;

        Some(ShapeHit {
            t: t_max,
            position,
            normal,
            u,
            v,
        })
    }
}
