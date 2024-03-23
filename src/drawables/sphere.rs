use crate::{
    scene::SceneObject,
    transformation::{Orientation, Transformable},
    vec3::Vec3,
};

pub struct Sphere {
    pos: Vec3,
    radius: f32,
    specular_intensity: f32,
    _orientation: Orientation,
}

impl Sphere {
    pub fn new(pos: Vec3, radius: f32, specular_intensity: f32, orientation: Orientation) -> Sphere {
        Sphere {
            pos,
            radius,
            specular_intensity,
            _orientation: orientation,
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
    
    fn specular_intensity(&self) -> f32 {
        self.specular_intensity
    }
}

impl Transformable for Sphere {
    fn rotate(&mut self, _x: f32, _y: f32, _z: f32) {
        todo!()
    }

    fn get_orientation(&self) -> &crate::transformation::Orientation {
        todo!()
    }

    fn translate(&mut self) {
        todo!()
    }
}
