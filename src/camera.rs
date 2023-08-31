use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    fov: u8,
    output_dimensions: (u32, u32),
}

impl Camera {
    pub fn new(origin: Vec3, fov: u8, output_dimensions: (u32, u32)) -> Camera {
        Camera {
            origin,
            fov,
            output_dimensions,
        }
    }

    pub fn calc_image_grid_distance(&self) -> f32 {
        1.0
    }
}
