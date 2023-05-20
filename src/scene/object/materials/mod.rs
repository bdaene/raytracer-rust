pub mod uniform;

use self::uniform::Uniform;
use palette::LinSrgb;

pub trait Material {
    fn get_color(&self) -> LinSrgb;
}

pub enum Materials {
    Uniform(Uniform),
}

impl Material for Materials {
    fn get_color(&self) -> LinSrgb {
        match self {
            Materials::Uniform(uniform) => uniform.get_color(),
        }
    }
}
