pub mod camera;
pub mod object;

use palette::LinSrgb;

use crate::utils::hit::{Hit, Hittable};
use crate::utils::ray::Ray;
use object::Object;

use self::camera::Camera;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub background: LinSrgb,
}

impl Hittable for Scene {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut t_max = t_max;
        let mut closest_hit = None;
        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, t_max) {
                t_max = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
