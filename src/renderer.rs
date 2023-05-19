use crate::pixels::Pixels;
use crate::scene::Scene;
use crate::utils::random;
use palette::LinSrgb;

pub struct Renderer {
    pub rays_per_pixel: usize,
}

impl Renderer {
    pub fn render(&self, scene: &Scene, pixels: &mut Pixels) {
        for row in 0..pixels.height {
            for col in 0..pixels.width {
                let mut color = LinSrgb::default();
                for _ in 0..self.rays_per_pixel {
                    let pixel_offset = random::get_random_2d();
                    let lens_offset = random::get_random_in_disk();

                    let u = (col as f64 + pixel_offset.0) / pixels.width as f64;
                    let v = (row as f64 + pixel_offset.1) / pixels.height as f64;

                    let ray = scene.camera.get_ray(lens_offset, (u, v));
                    color += if let Some((_hit, object)) = scene
                        .objects
                        .iter()
                        .filter_map(|object| {
                            object
                                .shape
                                .hit(ray, 0., f64::INFINITY)
                                .and_then(|hit| Some((hit, object)))
                        })
                        .min_by(|(a, _), (b, _)| a.t.partial_cmp(&b.t).expect("No NaN."))
                    {
                        object.material.get_color()
                    } else {
                        scene.background
                    }
                }
                color.red /= self.rays_per_pixel as f32;
                color.green /= self.rays_per_pixel as f32;
                color.blue /= self.rays_per_pixel as f32;

                pixels.set_color(row, col, color);
            }
        }
    }
}
