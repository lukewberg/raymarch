use std::simd::num::SimdUint;
use std::simd::prelude::SimdFloat;
use std::simd::{u8x4, Mask};
use std::{ops, simd::f32x4};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub vec: [f32; 4],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            vec: [x, y, z, 0.0],
        }
    }

    pub fn forward() -> Vec3 {
        Vec3 {
            vec: [0f32, 1_f32, 0f32, 0f32],
        }
    }

    pub fn up() -> Vec3 {
        Vec3 {
            vec: [0f32, 0f32, 1_f32, 0f32],
        }
    }

    pub fn right() -> Vec3 {
        Vec3 {
            vec: [1_f32, 0f32, 0f32, 0f32],
        }
    }

    pub fn x(&self) -> &f32 {
        &self.vec[0]
    }

    pub fn y(&self) -> &f32 {
        &self.vec[1]
    }

    pub fn z(&self) -> &f32 {
        &self.vec[2]
    }

    #[inline(always)]
    pub fn normalize(&mut self) {
        let mag = f32::sqrt(self.vec[0].powi(2) + self.vec[1].powi(2) + self.vec[2].powi(2));
        let mag_simd = f32x4::splat(mag);
        let vec_simd = f32x4::from_array(self.vec);
        self.vec = (vec_simd / mag_simd).into();
    }

    #[inline(always)]
    pub fn normalize_new(&self) -> Vec3 {
        let mag = f32::sqrt(self.vec[0].powi(2) + self.vec[1].powi(2) + self.vec[2].powi(2));
        let mag_simd = f32x4::splat(mag);
        let vec_simd = f32x4::from_array(self.vec);
        Vec3 {
            vec: (vec_simd / mag_simd).into(),
        }
    }

    #[inline(always)]
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.vec[0].powi(2) + self.vec[1].powi(2) + self.vec[2].powi(2))
        // f32::sqrt((f32x4::from_array(self.vec) * f32x4::from_array(self.vec)).reduce_sum())
    }

    /// Distance between this vector and that which is passed in
    #[inline(always)]
    pub fn distance(&self, p: &Vec3) -> f32 {
        // Can't use implemented operator overloads because we don't want to move values.
        let delta_vec: [f32; 4] =
            (f32x4::from_array((*self).vec) - f32x4::from_array((*p).vec)).into();
        let sum: [f32; 4] = (f32x4::from_array(delta_vec) * f32x4::from_array(delta_vec)).into();
        let result =
            (f32x4::splat(sum[0]) + f32x4::splat(sum[1]) + f32x4::splat(sum[2])).as_array()[0];
        result.sqrt()
    }

    #[inline(always)]
    pub fn scale(&self, scalar: f32) -> [f32; 4] {
        (f32x4::from_array(self.vec) * f32x4::splat(scalar)).into()
        // [
        //     self.vec[0] * scalar,
        //     self.vec[1] * scalar,
        //     self.vec[2] * scalar,
        //     0_f32
        // ]
    }

    #[inline(always)]
    pub fn multiply_vec3_simd(a: &Vec3, b: &Vec3) -> Vec3 {
        // Multiply using SIMD
        let vec_a = f32x4::from_array(a.vec);
        let vec_b = f32x4::from_array(b.vec);

        Vec3 {
            vec: (vec_a * vec_b).into(),
        }
    }

    #[inline(always)]
    pub fn multiply_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
        let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        result[0] = a.vec[0] * b.vec[0];
        result[1] = a.vec[1] * b.vec[1];
        result[2] = a.vec[2] * b.vec[2];
        Vec3 { vec: result }
    }

    #[inline(always)]
    pub fn multiply_vec3_scalar_simd(&self, scalar: f32) -> Vec3 {
        let vec_vec = f32x4::from_array(self.vec);
        let vec_scalar = f32x4::splat(scalar);
        Vec3 {
            vec: (vec_vec * vec_scalar).into(),
        }
    }

    #[inline(always)]
    pub fn multiply_vec3_scalar(&self, scalar: f32) -> Vec3 {
        Vec3 {
            vec: [
                self.vec[0] * scalar,
                self.vec[1] * scalar,
                self.vec[2] * scalar,
                0.0,
            ],
        }
    }

    #[inline(always)]
    pub fn abs(&self) -> Vec3 {
        Vec3 {
            vec: [self.vec[0].abs(), self.vec[1].abs(), self.vec[2].abs(), 0.0],
        }
    }

    #[inline(always)]
    pub fn combined_max(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            vec: [
                a.vec[0].max(b.vec[0]),
                a.vec[1].max(b.vec[1]),
                a.vec[2].max(b.vec[2]),
                0.0,
            ],
        }
    }

    #[inline(always)]
    pub fn combined_min(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            vec: [
                a.vec[0].min(b.vec[0]),
                a.vec[1].min(b.vec[1]),
                a.vec[2].min(b.vec[2]),
                0.0,
            ],
        }
    }

    #[inline(always)]
    pub fn dot_prod(&self, rhs: &Vec3) -> f32 {
        let mul = *self * *rhs;
        (f32x4::from_array(mul.vec)).reduce_sum()
    }

    // pub fn batch_multiply_vec3_simd(vecs_a: [Vec3; ], vecs_b: [Vec3]) -> Vec<Vec3> {
    //     Vec::new()
    // }

    /// Returns a tuple of the reflected vector and the vertical vector
    #[inline(always)]
    pub fn reflect(&self, normal: &Vec3) -> (Vec3, Vec3) {
        let v = (*normal * 2_f32) * ((*normal * -1_f32).dot_prod(self));
        let reflected = *self + v;
        (reflected, v)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        let vec_vec = f32x4::from_array(self.vec);
        let vec_scalar = f32x4::splat(rhs);
        Vec3 {
            vec: (vec_vec * vec_scalar).into(),
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let vec_self = f32x4::from_array(self.vec);
        let vec_vec = f32x4::from_array(rhs.vec);

        Vec3 {
            vec: (vec_self * vec_vec).into(),
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        let vec_scalar = f32x4::splat(rhs);
        let vec_vec = f32x4::from_array(self.vec);

        Vec3 {
            vec: (vec_scalar / vec_vec).into(),
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        let vec_self = f32x4::from_array(self.vec);
        let vec_vec = f32x4::from_array(rhs.vec);

        Vec3 {
            vec: (vec_self / vec_vec).into(),
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        let vec_scalar = f32x4::splat(rhs);
        let vec_vec = f32x4::from_array(self.vec);
        Vec3 {
            vec: (vec_scalar + vec_vec).into(),
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let lhs = f32x4::from_array(self.vec);
        let rhs_simd = f32x4::from_array(rhs.vec);
        Vec3 {
            vec: (lhs + rhs_simd).into(),
        }
    }
}

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        let vec_scalar = f32x4::splat(rhs);
        let vec_vec = f32x4::from_array(self.vec);
        let mask = (Mask::from_array([true, true, true, false])).select(vec_scalar, vec_vec);
        Vec3 {
            vec: (vec_vec - mask).into(),
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let vec_rhs = f32x4::from_array(rhs.vec);
        let vec_vec = f32x4::from_array(self.vec);
        Vec3 {
            vec: (vec_vec - vec_rhs).into(),
        }
    }
}

impl From<[u8; 4]> for Vec3 {
    fn from(value: [u8; 4]) -> Self {
        Vec3 {
            vec: u8x4::from_array(value).cast::<f32>().into(),
        }
    }
}

impl From<[f32; 4]> for Vec3 {
    fn from(value: [f32; 4]) -> Self {
        Vec3 { vec: value }
    }
}
