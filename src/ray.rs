use crate::{scene::Scene, vec3::Vec3};

pub struct Ray<'a> {
    origin: Vec3,
    direction: &'a mut Vec3,
    scene: &'a Scene,
}

impl Ray<'_> {
    pub fn new<'a>(origin: Vec3, direction: &'a mut Vec3, scene: &'a Scene) -> Ray<'a> {
        Ray {
            origin,
            direction,
            scene,
        }
    }

    pub fn cast(&self) -> u8 {
        // Get distances to all Objects in scene
        let num_scene_objects = self.scene.scene_objects.len();
        let mut distances: Vec<f32> = Vec::with_capacity(num_scene_objects);
        for i in 0..num_scene_objects {
            distances.push(self.scene.scene_objects[i].sdf());
        }
         
        2
    }
}
