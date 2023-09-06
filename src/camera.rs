use std::simd::f32x4;

use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    fov: (f32, f32),
    aspect_ratio: f32,
    output_dimensions: (u32, u32),
}

impl Camera {
    pub fn new(origin: Vec3, fov: f32, output_dimensions: (u32, u32)) -> Camera {
        let v_fov = 2.0 * ((fov / 2.0).tan() * (16_f32 / 9_f32)).atan();
        Camera {
            origin,
            fov: (fov, v_fov),
            aspect_ratio: 16_f32 / 9_f32,
            output_dimensions,
        }
    }

    pub fn calc_uv(&self) -> Vec<Vec<(f32, f32)>> {
        let mut result_vec = Vec::new();

        let mut y = 0;
        let mut x = 0;

        while y <= self.output_dimensions.1 {
            let mut x_vec = Vec::new();
            x_vec.resize(self.output_dimensions.0 as usize, (0_f32, 0_f32));

            while x <= self.output_dimensions.0 {
                // let x_splat = f32x4::splat(x as f32);
                // let y_splat = f32x4::splat(y as f32);
                // let u_simd = f32x4::from_array([x, x+1, x+2, x+3]);
                // let v_simd = f32x4::from_array([y, y+1, y+2, y+3]);
                let u = (x / self.output_dimensions.0) as f32 - 0.5; // Normalize to [-0.5, 0.5]
                let v = (y / self.output_dimensions.1) as f32 - 0.5; // Normalize to [-0.5, 0.5]
                x_vec[x as usize] = (u, v);
                x = x + 1;
            }
            result_vec[y as usize] = x_vec;
            y = y + 1;
        }
        result_vec
    }
}
