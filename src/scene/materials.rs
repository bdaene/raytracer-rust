use crate::utils::color::Color;

pub trait Material {
    fn get_color(&self) -> Color;
}

pub struct Uniform {
    pub color: Color,
}

impl Material for Uniform {
    fn get_color(&self) -> Color {
        self.color
    }
}
