pub mod color;
pub mod hit;
pub mod point;
pub mod random;
pub mod ray;

const REL_TOL: f64 = 1e-9;

pub fn is_close(a: f64, b: f64) -> bool {
    (a - b).abs() <= (a + b).abs() * REL_TOL
}
