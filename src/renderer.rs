use crate::pixels::Pixels;
use crate::scene::Scene;
use crate::utils::hit::Hittable;
use crate::utils::random;
use palette::LinSrgb;

pub struct Renderer {
    pub rays_per_pixel: usize,
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer {
            rays_per_pixel: 128,
        }
    }
}

impl Renderer {
    pub fn render(&self, scene: &Scene, pixels: &mut Pixels) {
        for row in 0..pixels.height {
            for col in 0..pixels.width {
                let color = self.render_pixel(scene, (row, col), (pixels.width, pixels.height));
                pixels.set_color(row, col, color);
            }
        }
    }

    fn render_pixel(
        &self,
        scene: &Scene,
        (row, col): (usize, usize),
        (screen_width, screen_height): (usize, usize),
    ) -> LinSrgb {
        let mut color = LinSrgb::default();
        for _ in 0..self.rays_per_pixel {
            let pixel_offset = random::get_random_2d();
            let u = (col as f64 + pixel_offset.0) / screen_width as f64;
            let v = (row as f64 + pixel_offset.1) / screen_height as f64;

            let lens_offset = random::get_random_in_disk();
            let ray = scene.camera.get_ray(lens_offset, (u, v));

            color += if let Some(hit) = scene.hit(ray, 0., f64::INFINITY) {
                hit.material.get_color()
            } else {
                scene.background
            };
        }
        color.red /= self.rays_per_pixel as f32;
        color.green /= self.rays_per_pixel as f32;
        color.blue /= self.rays_per_pixel as f32;

        color
    }
}
