use crate::{camera::Camera, light::Light, transformation::Transformable, vec3::Vec3, ray::Ray};

pub struct Scene {
    pub camera: Camera,
    // pub lights: Vec<Light>,
    pub scene_objects: Vec<Box<dyn SceneObject>>,
}

impl Scene {
    pub fn new(camera: Camera, scene_objects: Vec<Box<dyn SceneObject>>) -> Scene {
        Scene {
            camera,
            scene_objects,
        }
    }

    pub fn render(&self) {
        // Take the camera's UV grid and construct rays
        let uv = self.camera.calc_uv_simd();
        for i in 0..uv.coords.len() {
            let current_coords = uv.coords[i];
            let direction = self.camera.uv_direction(current_coords.0, current_coords.1).normalize();
            Ray::new(self.camera.origin, direction, &self);
        }
    }
}

pub trait SceneObject: Transformable {
    fn pos(&self) -> &Vec3;
    fn sdf(&self) -> f32;
}
