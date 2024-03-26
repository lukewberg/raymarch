use std::simd::{
    f32x4,
    num::{SimdFloat, SimdUint},
    u8x4,
};

use crate::{scene::SceneObject, vec3::Vec3};

pub struct Light {
    pub color: Vec3,
    pub pos: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn get_specular(
        light: &Light,
        scene_object: &Box<dyn SceneObject>,
        reflection_tup: (Vec3, Vec3),
    ) -> [u8; 4] {
        let (r, v) = reflection_tup;
        let light_color = light.color  / 255.0;
        let surface_color = scene_object.surface_color() / 255.0;
        let first = light_color * surface_color;
        let second = f32x4::splat(scene_object.specular_intensity() * r.dot_prod(&v).powi(1));
        let result = (f32x4::from_array(first.vec) * second * f32x4::splat(255.0))
        .cast::<u8>()
        .to_array();
        result
    }
}
