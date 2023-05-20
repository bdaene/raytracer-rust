use crate::scene::object::Object;
use crate::utils::ray::Ray;

pub struct Hit<'object> {
    pub object: &'object Object,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
