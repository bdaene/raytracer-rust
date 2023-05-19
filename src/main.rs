use std::path::Path;

use raytracer_rust::pixels::Pixels;
use raytracer_rust::renderer::Renderer;
use raytracer_rust::scene::camera::Camera;
use raytracer_rust::scene::materials::Uniform;
use raytracer_rust::scene::shapes::sphere::Sphere;
use raytracer_rust::scene::Object;
use raytracer_rust::scene::Scene;
use raytracer_rust::utils::color::Color;
use raytracer_rust::utils::point::Point;

fn main() {
    let x_offset = (std::f64::consts::PI / 3.0).sin();

    let scene = Scene {
        camera: Camera::new(
            Point::from_xyz(-2., 0., 2.5),
            Point::from_xyz(0., 0., 1.5),
            60.,
            16. / 9.,
            0.05,
        ),
        objects: vec![
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(0., 0., 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                    },
                }),
            },
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(0., 1., 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 1.,
                        green: 0.,
                        blue: 0.,
                    },
                }),
            },
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(-x_offset, 0.5, 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 1.,
                        green: 1.,
                        blue: 0.,
                    },
                }),
            },
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(-x_offset, -0.5, 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 0.,
                        green: 1.,
                        blue: 0.,
                    },
                }),
            },
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(0., -1., 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 0.,
                        green: 1.,
                        blue: 1.,
                    },
                }),
            },
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(x_offset, -0.5, 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 0.,
                        green: 0.,
                        blue: 1.,
                    },
                }),
            },
            Object {
                shape: Box::new(Sphere::new(Point::from_xyz(x_offset, 0.5, 1.5), 0.5)),
                material: Box::new(Uniform {
                    color: Color {
                        red: 1.,
                        green: 0.,
                        blue: 1.,
                    },
                }),
            },
        ],
    };

    let mut pixels = Pixels::new(1600, 900);
    let renderer = Renderer { rays_per_pixel: 128 };

    renderer.render(&scene, &mut pixels);

    pixels
        .save(Path::new("tmp/scene.png"))
        .expect("Should be saved.");
}