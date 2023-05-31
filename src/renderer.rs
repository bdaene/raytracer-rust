use std::time::Instant;

use crate::scene::object::materials::Materials;
use crate::scene::object::shapes::TOLERANCE;
use crate::scene::Scene;
use crate::utils::hit::{HitInfo, Hittable};
use crate::utils::random;
use crate::utils::ray::Ray;
use image::{Rgb, RgbImage};
use palette::{LinSrgb, Srgb};
use rayon::prelude::*;

pub struct Renderer {
    pub rays_per_pixel: usize,
    pub tile_size: usize,
    pub max_bounces: usize,
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer {
            rays_per_pixel: 512,
            tile_size: 16,
            max_bounces: 16,
        }
    }
}

impl Renderer {
    pub fn render(&self, scene: &Scene) -> RgbImage {
        let (width, height) = (scene.camera.screen_width, scene.camera.screen_height);
        let mut screen = RgbImage::new(width as u32, height as u32);

        let tiles: Vec<(usize, usize)> = (0..height)
            .step_by(self.tile_size)
            .flat_map(|row| {
                (0..width)
                    .step_by(self.tile_size)
                    .map(move |col| (row, col))
            })
            .collect();
        let start = Instant::now();
        println!("Computing {} tiles ", tiles.len());
        let pixels: Vec<(usize, usize, LinSrgb)> = tiles
            .into_par_iter()
            .flat_map(|(row, col)| {
                self.render_tile(
                    scene,
                    (width, height),
                    (row, col),
                    (
                        height.min(row + self.tile_size),
                        width.min(col + self.tile_size),
                    ),
                )
            })
            .collect();
        println!("Done in {:?}.", start.elapsed());

        for (row, col, color) in pixels {
            let color = Srgb::<u8>::from_linear(color * scene.camera.exposure);
            screen.put_pixel(
                col as u32,
                row as u32,
                Rgb([color.red, color.green, color.blue]),
            );
        }

        screen
    }

    fn render_tile(
        &self,
        scene: &Scene,
        screen_dimensions: (usize, usize),
        from: (usize, usize),
        to: (usize, usize),
    ) -> Vec<(usize, usize, LinSrgb)> {
        let mut pixels: Vec<(usize, usize, LinSrgb)> = Vec::new();

        for row in from.0..to.0 {
            for col in from.1..to.1 {
                pixels.push((
                    row,
                    col,
                    self.render_pixel(scene, screen_dimensions, (row, col)),
                ));
            }
        }

        pixels
    }

    fn render_pixel(
        &self,
        scene: &Scene,
        (screen_width, screen_height): (usize, usize),
        (row, col): (usize, usize),
    ) -> LinSrgb {
        let mut color = LinSrgb::default();
        for _ in 0..self.rays_per_pixel {
            let pixel_offset = random::get_random_2d();
            let u = (col as f64 + pixel_offset.0) / screen_width as f64;
            let v = (row as f64 + pixel_offset.1) / screen_height as f64;

            let lens_offset = random::get_random_in_disk();
            let ray = scene.camera.get_ray(lens_offset, (u, v));

            color += self.render_ray(scene, ray);
        }
        color.red /= self.rays_per_pixel as f32;
        color.green /= self.rays_per_pixel as f32;
        color.blue /= self.rays_per_pixel as f32;

        color
    }

    fn render_ray(&self, scene: &Scene, ray: Ray) -> LinSrgb {
        let mut ray = ray;
        let mut albedo = LinSrgb::new(1., 1., 1.);
        let mut color = LinSrgb::default();
        for _ in 0..self.max_bounces {
            if let Some(hit) = scene.hit(ray, TOLERANCE, f64::INFINITY) {
                let hit_info = HitInfo::from(hit);
                albedo *= hit_info.color;
                if let Materials::Light(_) = hit_info.hit.object.material {
                    color += albedo
                }
                if let Some(next_ray) = hit_info.next_ray {
                    ray = next_ray;
                } else {
                    return color;
                }
            }
        }
        color
    }
}
