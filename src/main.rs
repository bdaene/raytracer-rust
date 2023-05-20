use std::path::Path;

use palette::Srgb;
use raytracer_rust::pixels::Pixels;
use raytracer_rust::renderer::Renderer;
use raytracer_rust::scene::camera::Camera;
use raytracer_rust::scene::object::materials::uniform::Uniform;
use raytracer_rust::scene::object::materials::Materials;
use raytracer_rust::scene::object::shapes::sphere::Sphere;
use raytracer_rust::scene::object::shapes::Shapes;
use raytracer_rust::scene::object::Object;
use raytracer_rust::scene::Scene;
use raytracer_rust::utils::point::Point;

fn main() {
    let mut screen = Pixels::new(16 * 20, 9 * 20);
    let renderer = Renderer::default();

    let x_offset = (std::f64::consts::PI / 3.0).sin();

    let scene = Scene {
        background: Srgb::new(0.5, 0.7, 1.).into_linear(),
        camera: Camera::new(
            Point::from_xyz(-2., 0., 2.5),
            Point::from_xyz(0., 0., 1.5),
            60.,
            16. / 9.,
            0.05,
        ),
        objects: vec![
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(0., 0., 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(0.5, 0.5, 0.5).into_linear(),
                }),
            },
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(0., 1., 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(1., 0., 0.).into_linear(),
                }),
            },
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(-x_offset, 0.5, 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(0.5, 0.5, 0.).into_linear(),
                }),
            },
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(-x_offset, -0.5, 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(0., 1., 0.).into_linear(),
                }),
            },
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(0., -1., 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(0., 0.5, 0.5).into_linear(),
                }),
            },
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(x_offset, -0.5, 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(0., 0., 1.).into_linear(),
                }),
            },
            Object {
                shape: Shapes::Sphere(Sphere::new(Point::from_xyz(x_offset, 0.5, 1.5), 0.5)),
                material: Materials::Uniform(Uniform {
                    color: Srgb::new(0.5, 0., 0.5).into_linear(),
                }),
            },
        ],
    };

    renderer.render(&scene, &mut screen);

    screen
        .save(Path::new("tmp/scene.png"))
        .expect("Should be saved.");
}
