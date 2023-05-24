use palette::LinSrgb;

use crate::scene::object::Object;
use crate::utils::point::Point;
use crate::utils::ray::Ray;

pub struct Hit<'object> {
    pub ray: Ray,
    pub object: &'object Object,
    pub t: f64,
}

pub struct ShapeHit {
    pub position: Point,
    pub normal: Point,

    pub u: f64,
    pub v: f64,
}

pub struct HitInfo<'object> {
    pub hit: Hit<'object>,
    pub shape_hit: ShapeHit,

    pub color: LinSrgb,
    pub next_ray: Option<Ray>,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

impl<'object> From<Hit<'object>> for HitInfo<'object> {
    fn from(hit: Hit) -> HitInfo {
        hit.object.get_hit_info(hit)
    }
}
