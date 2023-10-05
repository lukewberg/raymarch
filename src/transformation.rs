use crate::vec3::Vec3;

pub trait Transformable {
    fn rotate(&mut self, x: f32, y: f32, z: f32);
    fn get_orientation(&self) -> &Orientation;
    fn translate(&mut self);
}

pub struct Orientation {
    up: Vec3,
    right: Vec3,
    forward: Vec3,
}

impl Orientation {
    pub fn new(up: Vec3, right: Vec3, forward: Vec3) -> Orientation {
        Orientation { up, right, forward }
    }

    pub fn default() -> Orientation {
        Orientation {
            up: Vec3::up(),
            right: Vec3::right(),
            forward: Vec3::forward(),
        }
    }
}
