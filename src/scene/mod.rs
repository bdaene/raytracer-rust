pub mod camera;
pub mod object;

use palette::{LinSrgb, Srgb};

use crate::utils::hit::{Hit, Hittable};
use crate::utils::ray::Ray;
use object::Object;

use self::camera::Camera;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

serde_with::serde_conv!(
    LinSrgbAsArray,
    LinSrgb,
    |color: &LinSrgb| {
        let color: Srgb = Srgb::from_linear(*color);
        [color.red, color.green, color.blue]
    },
    |value: [f32; 3]| -> Result<_, std::convert::Infallible> {
        let color = Srgb::new(value[0], value[1], value[2]).into_linear();
        Ok(color)
    }
);

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,

    #[serde_as(as = "LinSrgbAsArray")]
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
