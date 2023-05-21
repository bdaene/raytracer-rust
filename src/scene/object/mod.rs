pub mod materials;
pub mod shapes;

use shapes::{Shape, Shapes};

use crate::utils::hit::{Hit, Hittable};
use crate::utils::ray::Ray;

use self::materials::Materials;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub shape: Shapes,
    pub material: Materials,
}

impl Hittable for Object {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if let Some(shape_hit) = self.shape.hit(ray, t_min, t_max) {
            Some(Hit {
                object: self,
                t: shape_hit.t,
            })
        } else {
            None
        }
    }
}
