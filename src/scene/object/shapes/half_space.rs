use super::{Shape, ShapeHit, TOLERANCE};
use crate::utils::point::{Point, PointAsArray};
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HalfSpace {
    #[serde_as(as = "PointAsArray")]
    pub position: Point,
    #[serde_as(as = "PointAsArray")]
    pub normal: Point,
    #[serde_as(as = "PointAsArray")]
    pub u: Point,
    #[serde_as(as = "PointAsArray")]
    pub v: Point,
}

impl Shape for HalfSpace {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let d = self.normal.dot(&ray.direction);
        if d.abs() < TOLERANCE {
            return None;
        }

        let n = self.normal.dot(&(self.position - ray.origin));
        let t = n / d;

        if t_min < t && t < t_max {
            Some(t)
        } else {
            None
        }
    }

    fn get_hit_info(&self, ray: Ray, t: f64) -> ShapeHit {
        let position = ray.at_t(t);

        ShapeHit {
            position,
            normal: self.normal,
            u: self.u.dot(&position),
            v: self.v.dot(&position),
        }
    }
}
