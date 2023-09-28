use std::{
    fmt,
    ops::{self, Neg},
    simd::{f32x4, Simd},
};

use crate::vec3::Vec3;

pub struct Mat3 {
    rows: ([f32; 4], [f32; 4], [f32; 4]),
}

impl Mat3 {
    pub fn roll(angle: f32) -> Mat3 {
        Mat3 {
            rows: (
                [1_f32, 0f32, 0f32, 0f32],
                [0f32, f32::cos(angle), f32::sin(angle).neg(), 0f32],
                [0f32, f32::sin(angle), f32::cos(angle), 0f32],
            ),
        }
    }

    pub fn pich(angle: f32) -> Mat3 {
        Mat3 {
            rows: (
                [f32::cos(angle), 0f32, f32::sin(angle), 0f32],
                [0f32, 1_f32, 0f32, 0f32],
                [f32::sin(angle).neg(), 0f32, f32::cos(angle), 0f32],
            ),
        }
    }

    pub fn yaw(angle: f32) -> Mat3 {
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
        let mat3_1_simd: [f32; 4] = (f32x4::from_array(self.rows.0) * vec_simd).into();
        let mat3_2_simd: [f32; 4] = (f32x4::from_array(self.rows.1) * vec_simd).into();
        let mat3_3_simd: [f32; 4] = (f32x4::from_array(self.rows.2) * vec_simd).into();
        // From what I can find, M2 has 4 128b neon SIMD registers.
        // There does not appear to be a slick way to "rotate" the matrix in the registers
        /*
        Can't go from:
        [1, 5, 7]
        [3, 10, 4]
        [6, 3, 2]

        To:
        [1, 3, 6]
        [5, 10, 3]
        [7, 4, 2]
        very easily without loosing portability or performance
        */

        Vec3::new(
            mat3_1_simd.iter().sum(),
            mat3_2_simd.iter().sum(),
            mat3_3_simd.iter().sum(),
        )
    }
}
