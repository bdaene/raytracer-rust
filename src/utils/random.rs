use rand::prelude::*;

pub fn get_random_2d() -> (f64, f64) {
    (random::<f64>(), random::<f64>())
}

pub fn get_random_in_disk() -> (f64, f64) {
    let r = random::<f64>().sqrt();
    let alpha = random::<f64>() * std::f64::consts::TAU;
    (r * alpha.cos(), r * alpha.sin())
}
