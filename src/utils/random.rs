use rand::prelude::*;

use super::point::Point;

pub fn get_random_2d() -> (f64, f64) {
    (random::<f64>(), random::<f64>())
}

pub fn get_random_in_disk() -> (f64, f64) {
    let r = random::<f64>().sqrt();
    let alpha = random::<f64>() * std::f64::consts::TAU;
    (r * alpha.cos(), r * alpha.sin())
}

pub fn get_random_on_circle() -> (f64, f64) {
    let alpha = random::<f64>() * std::f64::consts::TAU;
    (alpha.cos(), alpha.sin())
}

pub fn get_random_on_sphere() -> Point {
    let phi = random::<f64>() * std::f64::consts::TAU;
    let sin_theta = (random::<f64>() - 0.5) * 2.;
    let cos_theta = (1.-sin_theta*sin_theta).sqrt();

    Point {
        coord: [
            phi.cos() * cos_theta,
            phi.sin() * cos_theta,
            sin_theta,
        ],
    }
}
