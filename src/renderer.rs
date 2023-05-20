use std::time::Instant;

use crate::pixels::Pixels;
use crate::scene::object::materials::Material;
use crate::scene::Scene;
use crate::utils::hit::Hittable;
use crate::utils::random;
use palette::LinSrgb;
use rayon::prelude::*;

pub struct Renderer {
    pub rays_per_pixel: usize,
    pub tile_size: usize,
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer {
            rays_per_pixel: 512,
            tile_size: 16,
        }
    }
}

impl Renderer {
    pub fn render(&self, scene: &Scene, screen: &mut Pixels) {
        let tiles: Vec<(usize, usize)> = (0..screen.height)
            .step_by(self.tile_size)
            .flat_map(|row| {
                (0..screen.width)
                    .step_by(self.tile_size)
                    .map(move |col| (row, col))
            })
            .collect();
        let start = Instant::now();
        print!("Computing {} tiles ", tiles.len());
        let pixels: Vec<(usize, usize, LinSrgb)> = tiles
            .into_par_iter()
            .flat_map(|(row, col)| {
                if col == 0 {
                    print!(".")
                }
                self.render_tile(
                    scene,
                    screen,
                    (row, col),
                    (
                        screen.height.min(row + self.tile_size),
                        screen.width.min(col + self.tile_size),
                    ),
                )
            })
            .collect();
        println!("done in {:?}.", start.elapsed());

        for (row, col, color) in pixels {
            screen.set_color(row, col, color);
        }
    }

    fn render_tile(
        &self,
        scene: &Scene,
        screen: &Pixels,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Vec<(usize, usize, LinSrgb)> {
        let mut pixels: Vec<(usize, usize, LinSrgb)> = Vec::new();

        for row in from.0..to.0 {
            for col in from.1..to.1 {
                pixels.push((row, col, self.render_pixel(scene, screen, (row, col))));
            }
        }

        pixels
    }

    fn render_pixel(&self, scene: &Scene, screen: &Pixels, (row, col): (usize, usize)) -> LinSrgb {
        let mut color = LinSrgb::default();
        for _ in 0..self.rays_per_pixel {
            let pixel_offset = random::get_random_2d();
            let u = (col as f64 + pixel_offset.0) / screen.width as f64;
            let v = (row as f64 + pixel_offset.1) / screen.height as f64;

            let lens_offset = random::get_random_in_disk();
            let ray = scene.camera.get_ray(lens_offset, (u, v));

            color += if let Some(hit) = scene.hit(ray, 0., f64::INFINITY) {
                hit.object.material.get_color()
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
