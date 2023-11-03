use std::{f32::consts::PI, simd::f32x4};

use crate::{transformation::Orientation, uv::Uv, vec3::Vec3};

pub struct Camera {
    pub origin: Vec3,
    orientation: Orientation,
    fov_scale: f32,
    aspect_ratio: f32,
    output_dimensions: (u32, u32),
}

impl Camera {
    pub fn new(origin: Vec3, fov: f32, output_dimensions: (u32, u32)) -> Camera {
        // let v_fov = 2.0 * ((fov / 2.0).tan() * (16_f32 / 9_f32)).atan();
        Camera {
            origin,
            orientation: Orientation::new(Vec3::up(), Vec3::right(), Vec3::forward()),
            fov_scale: f32::tan(fov * 0.5 * PI / 180_f32),
            aspect_ratio: output_dimensions.0 as f32 / output_dimensions.1 as f32,
            output_dimensions,
        }
    }

    pub fn calc_uv_simd(&self) -> Uv {
        let flattened_dimensions = self.output_dimensions.0 * self.output_dimensions.1;
        let mut result_vec = Vec::<(f32, f32)>::with_capacity(flattened_dimensions as usize);
        // result_vec.resize(flattened_dimensions as usize, (0_f32, 0_f32));

        const CHUNK_SIZE: u32 = 4; // SIMD register size
        let total_iterations = (flattened_dimensions + CHUNK_SIZE - 1) / CHUNK_SIZE;

        for i in (0..total_iterations).map(|x| x * CHUNK_SIZE) {
            let index_simd =
                f32x4::from_array([i as f32, (i + 1) as f32, (i + 2) as f32, (i + 3) as f32]);

            let y_simd = index_simd / f32x4::splat(self.output_dimensions.0 as f32);
            let x_simd = index_simd % f32x4::splat(self.output_dimensions.0 as f32);

            let u_simd =
                (x_simd / f32x4::splat(self.output_dimensions.0 as f32)) - f32x4::splat(1.0);
            let v_simd =
                (y_simd / f32x4::splat(self.output_dimensions.1 as f32)) - f32x4::splat(1.0);

            for j in 0..CHUNK_SIZE {
                if i + j < flattened_dimensions {
                    result_vec.push((u_simd[j as usize], v_simd[j as usize]));
                    // result_vec[(i + j) as usize] = (u_simd[j as usize], v_simd[j as usize]);
                }
            }
        }

        Uv::new(result_vec, self.output_dimensions)
    }

    pub fn calc_uv(&self) -> Uv {
        let flattened_dimensions = self.output_dimensions.0 * self.output_dimensions.1;
        let mut result_vec: Vec<(f32, f32)> = Vec::with_capacity(flattened_dimensions as usize);
        let mut i = 0_u32;
        while i < flattened_dimensions {
            let x = i % self.output_dimensions.0;
            let y = i / self.output_dimensions.0;

            let u = x as f32 / (self.output_dimensions.0 - 1) as f32;
            let v = y as f32 / (self.output_dimensions.1 - 1) as f32;
            // if (x == 0) {
            //     println!("X reached 0, i is :{}", i)
            // }
            result_vec.push((u, v));
            // let y = i % self.output_dimensions.1;
            i += 1;
        }
        Uv::new(result_vec, self.output_dimensions)
    }

    pub fn uv_to_screen(&self, uv: (f32, f32)) -> (f32, f32) {
        let x = (uv.0 - 0.5) * self.fov_scale * self.aspect_ratio;
        let y = (uv.1 - 0.5) * self.fov_scale;
        (x, y)
    }

    pub fn screen_to_world(&self, x: f32, y: f32) -> Vec3 {
        let right = f32x4::from_array(self.orientation.right.vec) * f32x4::splat(x);
        let up = f32x4::from_array(self.orientation.up.vec) * f32x4::splat(-y);
        let forward = f32x4::from_array(self.orientation.forward.vec);
        let origin = f32x4::from_array(self.origin.vec);

        let result: [f32; 4] = (origin + right + up + forward).into();

        Vec3 { vec: result }
    }

    // pub fn uv_direction(&self, u: f32, v: f32) -> Vec3 {
    //     let camera_orientation_matrix = Mat3 {
    //         rows: (
    //             self.orientation.right.vec,
    //             self.orientation.up.vec,
    //             self.orientation.forward.vec,
    //         ),
    //     };
    //     // You must manually normalize the result. (mutability and borrow checker)
    //     let mut vec = Vec3::new(
    //         u * self.fov_scale.0.tan() / self.aspect_ratio,
    //         v * self.fov_scale.1.tan(),
    //         1.0,
    //     );
    //     vec.normalize();
    //     let result = camera_orientation_matrix * vec;

    //     result
    // }

    pub fn rotate(&mut self) {}
}
