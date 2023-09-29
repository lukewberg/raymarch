use crate::{camera::Camera, light::Light, transformation::Transformable, vec3::Vec3};

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub scene_objects: Vec<Box<dyn SceneObject>>,
}

pub trait SceneObject: Transformable {
    fn pos(&self) -> &Vec3;
    fn sdf(&self) -> f32;
}
