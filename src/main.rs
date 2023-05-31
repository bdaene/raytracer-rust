use std::fs::File;
use std::path::Path;

use raytracer_rust::renderer::Renderer;
use raytracer_rust::scene::Scene;

fn main() {
    println!("Loading scene...");
    let scene_file = File::open("data/scene.json").unwrap();
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let screen = Renderer::default().render(&scene);

    screen.save(Path::new("tmp/scene.png")).unwrap();
}
