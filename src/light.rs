use std::simd::{f32x4, u8x4};

use crate::{scene::SceneObject, vec3::Vec3};

pub struct Light {
    pub color: [u8; 4],
    pub pos: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn get_specular(
        light: &Light,
        scene_object: &dyn SceneObject,
        reflection_tup: (Vec3, Vec3),
    ) -> [u8; 4] {
        let (r, v) = reflection_tup;
        ((u8x4::from_array(light.color) * u8x4::from_array(scene_object.surface_color()))
            * f32x4::splat((scene_object.specular_intensity() * r.dot_prod(&v).powi(1))))
        .into()
    }
}
