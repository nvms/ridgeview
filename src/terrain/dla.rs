use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::{Heightmap, GRID_SIZE};

struct DlaGrid {
    occupied: Vec<bool>,
    depth: Vec<u32>,
    size: usize,
}

impl DlaGrid {
    fn new(size: usize) -> Self {
        Self {
            occupied: vec![false; size * size],
            depth: vec![0; size * size],
            size,
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    fn set(&mut self, x: usize, y: usize, d: u32) {
        let i = self.idx(x, y);
        self.occupied[i] = true;
        self.depth[i] = d;
    }

    fn has_neighbor(&self, x: usize, y: usize) -> Option<u32> {
        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < self.size as i32 && ny >= 0 && ny < self.size as i32 {
                let ni = self.idx(nx as usize, ny as usize);
                if self.occupied[ni] {
                    return Some(self.depth[ni]);
                }
            }
        }
        None
    }
}

fn grow_dla(size: usize, walkers: usize, rng: &mut StdRng) -> DlaGrid {
    let mut grid = DlaGrid::new(size);

    // seed the center
    let cx = size / 2;
    let cy = size / 2;
    grid.set(cx, cy, 0);

    for _ in 0..walkers {
        let mut x = rng.gen_range(0..size);
        let mut y = rng.gen_range(0..size);

        let max_steps = size * size;
        for _ in 0..max_steps {
            if let Some(neighbor_depth) = grid.has_neighbor(x, y) {
                grid.set(x, y, neighbor_depth + 1);
                break;
            }

            let dir = rng.gen_range(0..4);
            match dir {
                0 if x > 0 => x -= 1,
                1 if x < size - 1 => x += 1,
                2 if y > 0 => y -= 1,
                3 if y < size - 1 => y += 1,
                _ => {}
            }
        }
    }

    grid
}

fn blur_heightmap(hmap: &mut Heightmap, passes: usize) {
    let size = hmap.size;
    for _ in 0..passes {
        let prev = hmap.data.clone();
        for y in 1..size - 1 {
            for x in 1..size - 1 {
                let sum = prev[(y - 1) * size + x]
                    + prev[(y + 1) * size + x]
                    + prev[y * size + (x - 1)]
                    + prev[y * size + (x + 1)]
                    + prev[(y - 1) * size + (x - 1)]
                    + prev[(y - 1) * size + (x + 1)]
                    + prev[(y + 1) * size + (x - 1)]
                    + prev[(y + 1) * size + (x + 1)]
                    + prev[y * size + x];
                hmap.data[y * size + x] = sum / 9.0;
            }
        }
    }
}

pub fn generate(seed: u32, walkers: usize, blur_passes: usize) -> Heightmap {
    let mut rng = StdRng::seed_from_u64(seed as u64);

    // multi-resolution: start small, upscale, add detail
    let small_size = 64;
    let small_walkers = walkers / 4;
    let small_grid = grow_dla(small_size, small_walkers, &mut rng);

    let mut hmap = Heightmap::new(GRID_SIZE);

    // upscale the small grid's depth values into the full heightmap
    let max_depth = small_grid.depth.iter().copied().max().unwrap_or(1).max(1) as f32;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let sx = x * small_size / GRID_SIZE;
            let sy = y * small_size / GRID_SIZE;
            let d = small_grid.depth[sy * small_size + sx] as f32;
            hmap.set(x, y, d / max_depth);
        }
    }

    // add fine detail at full resolution
    let detail_grid = grow_dla(GRID_SIZE, walkers, &mut rng);
    let detail_max = detail_grid.depth.iter().copied().max().unwrap_or(1).max(1) as f32;
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let d = detail_grid.depth[y * GRID_SIZE + x] as f32 / detail_max;
            let base = hmap.get(x, y);
            hmap.set(x, y, base * 0.6 + d * 0.4);
        }
    }

    blur_heightmap(&mut hmap, blur_passes);
    hmap.normalize();
    hmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_normalized() {
        let hmap = generate(42, 1000, 2);
        let min = hmap.data.iter().copied().fold(f32::INFINITY, f32::min);
        let max = hmap.data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        assert!((min - 0.0).abs() < 1e-5);
        assert!((max - 1.0).abs() < 1e-5);
    }

    #[test]
    fn deterministic_with_same_seed() {
        let a = generate(99, 500, 2);
        let b = generate(99, 500, 2);
        assert_eq!(a.data, b.data);
    }

    #[test]
    fn different_seeds_produce_different_output() {
        let a = generate(1, 500, 2);
        let b = generate(2, 500, 2);
        assert_ne!(a.data, b.data);
    }

    #[test]
    fn correct_dimensions() {
        let hmap = generate(0, 500, 1);
        assert_eq!(hmap.size, GRID_SIZE);
        assert_eq!(hmap.data.len(), GRID_SIZE * GRID_SIZE);
    }

    #[test]
    fn more_blur_produces_smoother_output() {
        let rough = generate(42, 1000, 0);
        let smooth = generate(42, 1000, 10);

        let roughness = |h: &Heightmap| -> f32 {
            let mut total = 0.0;
            let s = h.size;
            for y in 1..s {
                for x in 1..s {
                    let dx = h.get(x, y) - h.get(x - 1, y);
                    let dy = h.get(x, y) - h.get(x, y - 1);
                    total += dx.abs() + dy.abs();
                }
            }
            total
        };

        assert!(roughness(&smooth) < roughness(&rough));
    }
}
