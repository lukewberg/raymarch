use crate::vec3::Vec3;

pub struct Light {
    pub color: [u8; 4],
    pub pos: Vec3,
    pub intensity: f32
}
