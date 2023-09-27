use std::{ops::{self, Neg}, simd::f32x4};

use crate::vec3::Vec3;

pub struct Mat3 {
    rows: ([f32; 4], [f32; 4], [f32; 4]),
}

impl Mat3 {
    pub fn roll(&self, angle: f32) -> Mat3 {
        Mat3 {
            rows: (
                [1_f32, 0f32, 0f32, 0f32],
                [0f32, f32::cos(angle), f32::sin(angle).neg(), 0f32],
                [0f32, f32::sin(angle), f32::cos(angle), 0f32],
            ),
        }
    }

    pub fn pich(&self, angle: f32) -> Mat3 {
        Mat3 {
            rows: (
                [f32::cos(angle), 0f32, f32::sin(angle), 0f32],
                [0f32, 1_f32, 0f32, 0f32],
                [f32::sin(angle).neg(), 0f32, f32::cos(angle), 0f32],
            ),
        }
    }

    pub fn yaw(&self, angle: f32) -> Mat3 {
        Mat3 {
            rows: (
                [f32::cos(angle), f32::sin(angle).neg(), 0f32, 0f32],
                [f32::sin(angle), f32::cos(angle), 0f32, 0f32],
                [0f32, 0f32, 1_f32, 0f32],
            ),
        }
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        // Load vector into SIMD register
        let vec_simd = f32x4::from_array(rhs.vec);
        // Load each row from matrix into SIMD registers
        let mat3_1_simd = f32x4::from_array(self.rows.0);
        let mat3_2_simd = f32x4::from_array(self.rows.1);
        let mat3_3_simd = f32x4::from_array(self.rows.2);
        // From what I can find, M2 has 4 128b neon SIMD registers.
        

    }
}
