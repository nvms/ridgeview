pub mod dla;
pub mod gradient;
pub mod mesh;

pub const GRID_SIZE: usize = 256;

pub struct Heightmap {
    pub data: Vec<f32>,
    pub size: usize,
}

impl Heightmap {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size * size],
            size,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[y * self.size + x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: f32) {
        self.data[y * self.size + x] = val;
    }

    pub fn normalize(&mut self) {
        let min = self.data.iter().copied().fold(f32::INFINITY, f32::min);
        let max = self.data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let range = max - min;
        if range > 0.0 {
            for v in &mut self.data {
                *v = (*v - min) / range;
            }
        }
    }
}
