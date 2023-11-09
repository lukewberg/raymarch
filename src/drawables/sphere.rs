use crate::{
    scene::SceneObject,
    transformation::{Orientation, Transformable},
    vec3::Vec3,
};

pub struct Sphere {
    pos: Vec3,
    radius: f32,
    orientation: Orientation,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32, orientation: Orientation) -> Sphere {
        Sphere {
            pos,
            radius,
            orientation,
        }
    }
}

impl SceneObject for Sphere {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn sdf(&self, p: &Vec3) -> f32 {
        // self.pos.magnitude() - self.radius
        p.distance(&self.pos) - self.radius
    }
}

impl Transformable for Sphere {
    fn rotate(&mut self, x: f32, y: f32, z: f32) {
        todo!()
    }

    fn get_orientation(&self) -> &crate::transformation::Orientation {
        todo!()
    }

    fn translate(&mut self) {
        todo!()
    }
}
