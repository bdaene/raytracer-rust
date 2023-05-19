use std::iter::zip;
use std::ops;

use super::is_close;

pub const DIMENSIONS: usize = 3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub coord: [f64; DIMENSIONS],
}

impl Point {
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Point {
        Point { coord: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.coord[0]
    }

    pub fn y(&self) -> f64 {
        self.coord[1]
    }

    pub fn z(&self) -> f64 {
        self.coord[2]
    }

    pub fn dot(&self, other: &Point) -> f64 {
        zip(self.coord, other.coord).map(|(x1, x2)| x1 * x2).sum()
    }

    pub fn cross(&self, other: &Point) -> Point {
        Point::from_xyz(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn norm_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalized(&self) -> Point {
        *self / self.norm()
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let mut coord = self.coord.clone();
        for (i, x) in other.coord.iter().enumerate() {
            coord[i] += x;
        }

        Point { coord }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        let mut coord = self.coord.clone();
        for (i, x) in other.coord.iter().enumerate() {
            coord[i] -= x;
        }

        Point { coord }
    }
}

impl ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scale: f64) -> Point {
        let mut coord = self.coord.clone();
        for x in coord.iter_mut() {
            *x *= scale
        }

        Point { coord }
    }
}

impl ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        let mut coord = point.coord.clone();
        for x in coord.iter_mut() {
            *x *= self
        }

        Point { coord }
    }
}

impl ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, scale: f64) -> Point {
        let mut coord = self.coord.clone();
        for x in coord.iter_mut() {
            *x /= scale
        }

        Point { coord }
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        self * -1.
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        zip(self.coord, other.coord).all(|(x1, x2)| is_close(x1, x2))
    }

    fn ne(&self, other: &Self) -> bool {
        !Point::eq(&self, &other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(
            Point::default(),
            Point {
                coord: [0.; DIMENSIONS]
            }
        );
    }

    #[test]
    fn test_xyz() {
        let (x, y, z) = (1., 2., 3.);
        let point = Point {
            coord: [1., 2., 3.],
        };

        assert_eq!(Point::from_xyz(x, y, z), point);
        assert_eq!(point.x(), x);
        assert_eq!(point.y(), y);
        assert_eq!(point.z(), z);
    }

    #[test]
    fn test_dot() {
        let point_a = Point {
            coord: [1., 2., 3.],
        };
        let point_b = Point {
            coord: [4., 5., 6.],
        };

        assert_eq!(point_a.dot(&point_b), 32.);
        assert_eq!(point_b.dot(&point_a), 32.);
    }

    #[test]
    fn test_cross() {
        let point_a = Point {
            coord: [1., 2., 3.],
        };
        let point_b = Point {
            coord: [4., 5., 6.],
        };

        assert_eq!(point_a.cross(&point_b), Point::from_xyz(-3., 6., -3.));
        assert_eq!(point_b.cross(&point_a), -Point::from_xyz(-3., 6., -3.));
    }

    #[test]
    fn test_norm_squared() {
        let point = Point::from_xyz(1., 2., 3.);

        assert_eq!(point.norm_squared(), 14.);
    }

    #[test]
    fn test_norm() {
        let point = Point::from_xyz(2., 3., 6.);

        assert_eq!(point.norm(), 7.);
    }

    #[test]
    fn test_normalized() {
        let point = Point::from_xyz(2., 3., 6.);

        assert_eq!(
            point.normalized(),
            Point::from_xyz(2. / 7., 3. / 7., 6. / 7.)
        );
    }

    #[test]
    fn test_ops() {
        let point_a = Point {
            coord: [1., 2., 7.],
        };
        let point_b = Point {
            coord: [6., 2., 3.],
        };

        assert_eq!(point_a + point_b, Point::from_xyz(7., 4., 10.));
        assert_eq!(point_a - point_b, Point::from_xyz(-5., 0., 4.));
        assert_eq!(point_a * 3., Point::from_xyz(3., 6., 21.));
        assert_eq!(3. * point_a, Point::from_xyz(3., 6., 21.));
        assert_eq!(point_a / 2., Point::from_xyz(0.5, 1., 3.5));
    }
}
