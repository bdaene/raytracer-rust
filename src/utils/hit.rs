use crate::utils::ray::Ray;

pub struct Hit {
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
