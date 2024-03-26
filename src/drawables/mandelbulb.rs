use crate::{scene::SceneObject, transformation::Transformable, vec3::Vec3};

pub struct Mandelbulb {
    pos: Vec3,
    power: f32,
    color: Vec3,
    specular_intensity: f32,
}

impl Mandelbulb {
    pub fn new(pos: Vec3, power: f32, specular_intensity: f32) -> Mandelbulb {
        Mandelbulb {
            pos,
            power,
            color: [255, 255, 255, 0].into(),
            specular_intensity,
        }
    }
}

impl SceneObject for Mandelbulb {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn sdf(&self, p: &Vec3) -> f32 {
        let mut z = p.clone();
        let mut dr = 1.0;
        let mut r: f32 = 0_f32;
        for _i in 0..90 {
            r = z.magnitude();

            if r > 4.0 {
                break;
            };

            // convert to polar coordinates
            let mut theta = f32::acos(z.vec[2] / r);
            // let mut theta = (z.vec[2] / r).acos();
            let mut phi = f32::atan2(z.vec[1], z.vec[0]);
            // let mut phi = z.vec[1].atan2(z.vec[0]);
            // dr =  pow( r, self.power-1.0)*Power*dr + 1.0;
            dr = f32::powf(r, self.power - 1.0) * self.power * dr + 1.0;
            // dr = r.powf(self.power - 1.0) * self.power * dr + 1.0;
            // scale and rotate the point
            let zr = f32::powf(r, self.power);
            // let zr = r.powf(self.power);
            theta = theta * self.power;
            phi = phi * self.power;

            // convert back to cartesian coordinates
            z = Vec3::new(
                theta.sin() * phi.cos(),
                phi.sin() * theta.sin(),
                theta.cos(),
            );

            z.vec = z.scale(zr);
            z = z + *p;
        }

        return 0.5 * r.ln() * r / dr;
    }

    fn specular_intensity(&self) -> f32 {
        self.specular_intensity
    }

    fn surface_color(&self) -> Vec3 {
        todo!()
    }
}

impl Transformable for Mandelbulb {
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
