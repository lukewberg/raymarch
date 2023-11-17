use crate::{scene::SceneObject, transformation::Transformable, vec3::Vec3};

pub struct Cube {
    pos: Vec3,
    bounds: Vec3,
}

impl Cube {
    pub fn new(pos: Vec3, bounds: Vec3) -> Cube {
        Cube {
            pos,
            bounds
        }
    }
}

impl SceneObject for Cube {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn sdf(&self, p: &Vec3) -> f32 {
        let shiftPt = *p - self.pos;
        let q = shiftPt.abs() - self.bounds;
        Vec3::combined_max(&q, &Vec3::new(0.0, 0.0, 0.0)).magnitude()
            + 0_f32.min(q.vec[0].max(q.vec[1].max(q.vec[2])))
    }
}

impl Transformable for Cube {
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
