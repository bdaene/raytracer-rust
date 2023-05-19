use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn at_t(&self, t:f64) -> Point {
        self.origin + t * self.direction
    }
}