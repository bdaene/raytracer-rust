pub mod camera;
pub mod materials;
pub mod shapes;

use materials::Material;
use shapes::Shape;

use self::camera::Camera;

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
}
