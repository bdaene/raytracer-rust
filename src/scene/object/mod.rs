pub mod materials;
pub mod shapes;

use materials::Material;
use shapes::Shape;

use crate::utils::hit::{Hit, Hittable};
use crate::utils::ray::Ray;

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
}

impl Hittable for Object {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if let Some(shape_hit) = self.shape.hit(ray, t_min, t_max) {
            Some(Hit {
                material: &self.material,
                t: shape_hit.t,
            })
        } else {
            None
        }
    }
}
