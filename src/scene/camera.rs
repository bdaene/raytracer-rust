use crate::utils::point::Point;
use crate::utils::ray::Ray;

#[derive(Debug, PartialEq)]
pub struct Camera {
    position: Point,
    target: Point,
    up: Point,
    left: Point,

    screen_origin: Point,
    screen_horizontal: Point,
    screen_vertical: Point,
}

impl Camera {
    pub fn new(
        position: Point,
        target: Point,
        field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
    ) -> Camera {
        let vertical = Point::from_xyz(0., 0., 1.);
        let to_target = (target - position).normalized();
        let left = vertical.cross(&to_target).normalized();
        let up = to_target.cross(&left).normalized();

        let depth_of_field = (target - position).norm();
        let screen_width = depth_of_field * (field_of_view.to_radians() / 2.).tan() * 2.;
        let screen_height = screen_width / aspect_ratio;
        let screen_horizontal = -left * screen_width;
        let screen_vertical = -up * screen_height;
        let screen_origin = target - (screen_horizontal + screen_vertical) / 2.0;

        Camera {
            position,
            target,
            up: up * aperture / 2.,
            left: left * aperture / 2.,

            screen_origin,
            screen_horizontal,
            screen_vertical,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, lens_offset: (f64, f64), screen_offset: (f64, f64)) -> Ray {
        let origin = self.position + self.left * lens_offset.0 + self.up * lens_offset.1;
        let target = self.screen_origin
            + self.screen_horizontal * screen_offset.0
            + self.screen_vertical * screen_offset.1;

        Ray {
            origin,
            direction: target - origin,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Camera::new(
                Point::from_xyz(0., 0., 1.5),
                Point::from_xyz(8., 0., 1.5),
                90.,
                16. / 9.,
                0.04
            ),
            Camera {
                position: Point::from_xyz(0., 0., 1.5),
                target: Point::from_xyz(8., 0., 1.5),
                up: Point::from_xyz(0., 0., 0.04),
                left: Point::from_xyz(0., 0.04, 0.),

                screen_origin: Point::from_xyz(8., -8., 1.5 - 4.5),
                screen_horizontal: Point::from_xyz(0., 16., 0.),
                screen_vertical: Point::from_xyz(0., 0., 9.),
            }
        )
    }

    #[test]
    fn test_get_ray() {
        let camera = Camera::new(
            Point::from_xyz(0., 0., 1.5),
            Point::from_xyz(8., 0., 1.5),
            90.,
            16. / 9.,
            0.04,
        );

        assert_eq!(
            camera.get_ray((0., 0.), (0.5, 0.5)),
            Ray {
                origin: camera.position,
                direction: camera.target - camera.position
            }
        );
        assert_eq!(
            camera.get_ray((-1., 1.), (0.25, 0.75)),
            Ray {
                origin: Point::from_xyz(0., -0.04, 1.5 + 0.04),
                direction: Point::from_xyz(8., 16. / 4. + 0.04, -9. / 4. - 0.04),
            }
        );
    }
}