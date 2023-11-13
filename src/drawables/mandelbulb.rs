use crate::{
    scene::SceneObject,
    transformation::{Orientation, Transformable},
    vec3::Vec3,
};

pub struct Mandelbulb {
    pos: Vec3,
    power: f32,
}

impl Mandelbulb {
    pub fn new(pos: Vec3, power: f32) -> Mandelbulb {
        Mandelbulb { pos, power }
    }
}

impl SceneObject for Mandelbulb {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn sdf(&self, p: &Vec3) -> f32 {
        let mut z = *p;
        let mut dr = 1_f32;
        let mut r: f32 = 0_f32;
        for i in 0..20 {
            r = z.magnitude();

            if r > 4.0 {
                break;
            };

            // convert to polar coordinates
            let mut theta = (z.vec[2] / r).acos();
            let mut phi = z.vec[1].atan2(z.vec[0]);
            // dr =  pow( r, self.power-1.0)*Power*dr + 1.0;
            dr = r.powf(self.power - 1_f32) * self.power * dr + 1_f32;
            // scale and rotate the point
            let zr = r.powf(self.power);
            theta = theta * self.power;
            phi = phi * self.power;

            // convert back to cartesian coordinates
            z = Vec3::new(
                theta.sin() * phi.cos(),
                phi.sin() * theta.sin(),
                theta.cos(),
            ) * zr;
            z = z + self.pos;
        }

        return 0.5 * r.ln() * r / dr;
    }
}

impl Transformable for Mandelbulb {
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
