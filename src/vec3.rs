use std::simd::f32x4;

pub struct Vec3 {
    pub vec: [f32; 4],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            vec: [x, y, z, 0.0],
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

    pub fn multiply_vec3_simd(a: &Vec3, b: &Vec3) -> Vec3 {
        // Multiply using SIMD
        let vecA = f32x4::from_array(a.vec);
        let vecB = f32x4::from_array(b.vec);

        return Vec3 {
            vec: (vecA * vecB).into(),
        };
    }

    pub fn multiply_vec3(a: &Vec3, b: &Vec3) -> Vec3 {
        let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        result[0] = a.vec[0] * b.vec[0];
        result[1] = a.vec[1] * b.vec[1];
        result[2] = a.vec[2] * b.vec[2];
        Vec3 { vec: result }
    }
}
