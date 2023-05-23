use crate::scene::object::Object;
use crate::scene::object::shapes::ShapeHit;
use crate::utils::ray::Ray;

pub struct Hit<'object> {
    pub object: &'object Object,
    pub shape_hit: ShapeHit,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
