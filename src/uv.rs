use std::ops;

pub struct UV {
    pub coords: Vec<(f32, f32)>,
    pub dimensions: (u32, u32),
}

impl UV {
    pub fn new(coords: Vec<(f32, f32)>, dimensions: (u32, u32)) -> Self {
        UV { coords, dimensions }
    }
}

impl ops::Index<(u32, u32)> for UV {
    type Output = (f32, f32);

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.coords[(index.1 * self.dimensions.0 + index.0) as usize]
    }
}
