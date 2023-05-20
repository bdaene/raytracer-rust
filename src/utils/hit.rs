use crate::scene::object::materials::Material;
use crate::utils::ray::Ray;

pub struct Hit<'material> {
    pub t: f64,
    pub material: &'material Box<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
