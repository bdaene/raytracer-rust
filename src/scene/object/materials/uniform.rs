use super::Material;
use palette::LinSrgb;

pub struct Uniform {
    pub color: LinSrgb,
}

impl Material for Uniform {
    fn get_color(&self) -> LinSrgb {
        self.color
    }
}
