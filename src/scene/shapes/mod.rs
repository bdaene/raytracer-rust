pub mod sphere;

use crate::utils::hit::Hittable;

pub trait Shape: Hittable {}
