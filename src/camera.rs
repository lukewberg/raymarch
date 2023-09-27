use std::simd::f32x4;

use crate::{uv::UV, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    top: Vec3,
    right: Vec3,
    forward: Vec3,
    fov: (f32, f32),
    aspect_ratio: f32,
    output_dimensions: (u32, u32),
}

impl Camera {
    pub fn new(origin: Vec3, fov: f32, output_dimensions: (u32, u32)) -> Camera {
        let v_fov = 2.0 * ((fov / 2.0).tan() * (16_f32 / 9_f32)).atan();
        Camera {
            origin,
            top: Vec3::top(),
            right: Vec3::right(),
            forward: Vec3::forward(),
            fov: (fov, v_fov),
            aspect_ratio: 16_f32 / 9_f32,
            output_dimensions,
        }
    }

    pub fn calc_uv_simd(&self) -> UV {
        let flattened_dimensions = self.output_dimensions.0 * self.output_dimensions.1;
        let mut result_vec = Vec::<(f32, f32)>::with_capacity(flattened_dimensions as usize);
        // result_vec.resize(flattened_dimensions as usize, (0_f32, 0_f32));

        const CHUNK_SIZE: u32 = 4; // SIMD register size
        let total_iterations = (flattened_dimensions + CHUNK_SIZE - 1) / CHUNK_SIZE;

        for i in (0..total_iterations).map(|x| x * CHUNK_SIZE) {
            let index_simd =
                f32x4::from_array([i as f32, (i + 1) as f32, (i + 2) as f32, (i + 3) as f32]);

            let y_simd = index_simd % f32x4::splat(self.output_dimensions.1 as f32);
            let x_simd = index_simd / f32x4::splat(self.output_dimensions.1 as f32);

            let u_simd =
                (x_simd / f32x4::splat(self.output_dimensions.0 as f32)) - f32x4::splat(0.5);
            let v_simd =
                (y_simd / f32x4::splat(self.output_dimensions.1 as f32)) - f32x4::splat(0.5);

            for j in 0..CHUNK_SIZE {
                if i + j < flattened_dimensions {
                    result_vec.push((u_simd[j as usize], v_simd[j as usize]));
                    // result_vec[(i + j) as usize] = (u_simd[j as usize], v_simd[j as usize]);
                }
            }
        }

        UV::new(result_vec, self.output_dimensions)
    }

    pub fn rotate(&mut self) {}
}
