use crate::utils::point::{Point, PointAsArray};
use crate::utils::ray::Ray;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(from = "CameraConfig", into = "CameraConfig")]
pub struct Camera {
    position: Point,
    target: Point,
    up: Point,
    left: Point,

    screen_origin: Point,
    screen_horizontal: Point,
    screen_vertical: Point,

    pub screen_width: usize,
    pub screen_height: usize,
    pub exposure: f32,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct CameraConfig {
    #[serde_as(as = "PointAsArray")]
    position: Point,
    #[serde_as(as = "PointAsArray")]
    target: Point,
    #[serde_as(as = "PointAsArray")]
    up: Point,

    field_of_view: f64,
    aperture: f64,
    exposure: f32,

    screen_width: usize,
    screen_height: usize,
}

impl From<Camera> for CameraConfig {
    fn from(camera: Camera) -> CameraConfig {
        let depth_of_view = (camera.target - camera.position).norm();
        let field_of_view = (camera.screen_horizontal.norm() / depth_of_view / 2.)
            .atan()
            .to_degrees()
            * 2.;

        CameraConfig {
            position: camera.position,
            target: camera.target,
            up: camera.up,
            field_of_view,
            aperture: camera.up.norm() * 2.,
            exposure: camera.exposure,

            screen_width: camera.screen_width,
            screen_height: camera.screen_height,
        }
    }
}

impl From<CameraConfig> for Camera {
    fn from(config: CameraConfig) -> Camera {
        Camera::new(
            config.position,
            config.target,
            config.up,
            config.field_of_view,
            config.aperture,
            config.exposure,
            config.screen_width,
            config.screen_height,
        )
    }
}

impl Camera {
    pub fn new(
        position: Point,
        target: Point,
        up: Point,
        field_of_view: f64,
        aperture: f64,
        exposure: f32,
        screen_width: usize,
        screen_height: usize,
    ) -> Camera {
        let to_target = (target - position).normalized();
        let left = up.cross(&to_target).normalized();
        let up = to_target.cross(&left).normalized();

        let depth_of_field = (target - position).norm();
        let screen_horizontal_norm = depth_of_field * (field_of_view.to_radians() / 2.).tan() * 2.;
        let screen_vertical_norm =
            screen_horizontal_norm / (screen_width as f64 / screen_height as f64);
        let screen_horizontal = -left * screen_horizontal_norm;
        let screen_vertical = -up * screen_vertical_norm;
        let screen_origin = target - (screen_horizontal + screen_vertical) / 2.0;

        Camera {
            position,
            target,
            up: up * aperture / 2.,
            left: left * aperture / 2.,

            screen_origin,
            screen_horizontal,
            screen_vertical,

            screen_width,
            screen_height,
            exposure,
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
                Point::from_xyz(0., 0., 1.),
                90.,
                0.04,
                1.0,
                1600,
                900,
            ),
            Camera {
                position: Point::from_xyz(0., 0., 1.5),
                target: Point::from_xyz(8., 0., 1.5),
                up: Point::from_xyz(0., 0., 0.02),
                left: Point::from_xyz(0., 0.02, 0.),

                screen_origin: Point::from_xyz(8., 8., 1.5 + 4.5),
                screen_horizontal: Point::from_xyz(0., -16., 0.),
                screen_vertical: Point::from_xyz(0., 0., -9.),

                screen_width: 1600,
                screen_height: 900,
                exposure: 1.0,
            }
        )
    }

    #[test]
    fn test_get_ray() {
        let camera = Camera::new(
            Point::from_xyz(0., 0., 1.5),
            Point::from_xyz(8., 0., 1.5),
            Point::from_xyz(0., 0., 1.),
            90.,
            0.04,
            1.0,
            1600,
            900,
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
                origin: Point::from_xyz(0., -0.02, 1.5 + 0.02),
                direction: Point::from_xyz(8., 16. / 4. + 0.02, -9. / 4. - 0.02),
            }
        );
    }
}
