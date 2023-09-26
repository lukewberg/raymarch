use std::{ops, simd::f32x4};

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

    pub fn top() -> Vec3 {
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

    pub fn normalize(&mut self) {
        let mag = f32::sqrt(self.vec[0].powi(2) + self.vec[1].powi(2) + self.vec[2].powi(2));
        let mag_simd = f32x4::splat(mag);
        let vec_simd = f32x4::from_array(self.vec);
        self.vec = (vec_simd / mag_simd).into();
    }

    // #[inline(never)]
    pub fn multiply_vec3_simd(a: &Vec3, b: &Vec3) -> Vec3 {
        // Multiply using SIMD
        let vecA = f32x4::from_array(a.vec);
        let vecB = f32x4::from_array(b.vec);

        Vec3 {
            vec: (vecA * vecB).into(),
        }
    }

    pub fn multiply_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
        let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        result[0] = a.vec[0] * b.vec[0];
        result[1] = a.vec[1] * b.vec[1];
        result[2] = a.vec[2] * b.vec[2];
        Vec3 { vec: result }
    }

    // #[inline(never)]
    pub fn multiply_vec3_scalar_simd(&self, scalar: f32) -> Vec3 {
        let vec_vec = f32x4::from_array(self.vec);
        let vec_scalar = f32x4::splat(scalar);
        Vec3 {
            vec: (vec_vec * vec_scalar).into(),
        }
    }

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

    // pub fn batch_multiply_vec3_simd(vecs_a: [Vec3; ], vecs_b: [Vec3]) -> Vec<Vec3> {
    //     Vec::new()
    // }
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

impl ops::Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f32) -> Self::Output {
        let vec_scalar = f32x4::splat(rhs);
        let vec_vec = f32x4::from_array(self.vec);
        Vec3 {
            vec: (vec_scalar - vec_vec).into(),
        }
    }
}
