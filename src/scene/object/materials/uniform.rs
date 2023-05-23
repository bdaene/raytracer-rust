use super::Material;
use crate::scene::LinSrgbAsArray;
use crate::utils::hit::Hit;
use palette::LinSrgb;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Uniform {
    #[serde_as(as = "LinSrgbAsArray")]
    pub color: LinSrgb,
}

impl Material for Uniform {
    fn get_color(&self, _hit: Hit) -> LinSrgb {
        self.color
    }
}
