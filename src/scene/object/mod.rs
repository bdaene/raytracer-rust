pub mod colors;
pub mod materials;
pub mod shapes;

use self::materials::{Material, Materials};
use crate::utils::hit::{Hit, HitInfo, Hittable};
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};
use shapes::{Shape, Shapes};

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub shape: Shapes,
    pub material: Materials,
}

impl Hittable for Object {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if let Some(t) = self.shape.hit(ray, t_min, t_max) {
            Some(Hit {
                object: self,
                t,
                ray,
            })
        } else {
            None
        }
    }
}

impl<'object> Object {
    pub fn get_hit_info(&self, hit: Hit<'object>) -> HitInfo<'object> {
        let shape_hit = self.shape.get_hit_info(hit.ray, hit.t);
        let (color, next_ray) = self.material.bounce_ray(hit.ray, hit.t, &shape_hit);

        HitInfo {
            hit,
            shape_hit,
            color,
            next_ray,
        }
    }
}
