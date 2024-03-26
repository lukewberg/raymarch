use crate::{scene::SceneObject, transformation::Transformable, vec3::Vec3};

pub struct Cube {
    pos: Vec3,
    color: Vec3,
    specular_intensity: f32,
    bounds: Vec3,
}

impl Cube {
    pub fn new(pos: Vec3, specular_intensity: f32, bounds: Vec3) -> Cube {
        Cube {
            pos,
            color: [255, 255, 255, 0].into(),
            specular_intensity,
            bounds,
        }
    }
}

impl SceneObject for Cube {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn sdf(&self, p: &Vec3) -> f32 {
        let shift_pt = *p - self.pos;
        let q = shift_pt.abs() - self.bounds;
        Vec3::combined_max(&q, &Vec3::new(0.0, 0.0, 0.0)).magnitude()
            + 0_f32.min(q.vec[0].max(q.vec[1].max(q.vec[2])))
    }

    fn specular_intensity(&self) -> f32 {
        self.specular_intensity
    }

    fn surface_color(&self) -> Vec3 {
        self.color
    }
}

impl Transformable for Cube {
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
