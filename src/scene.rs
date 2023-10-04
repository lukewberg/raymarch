use crate::{camera::Camera, light::Light, transformation::Transformable, vec3::Vec3};

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
        
    }
}

pub trait SceneObject: Transformable {
    fn pos(&self) -> &Vec3;
    fn sdf(&self) -> f32;
}
