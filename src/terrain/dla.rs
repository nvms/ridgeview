use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::{Heightmap, GRID_SIZE};

fn grow_dla(size: usize, walkers: usize, rng: &mut StdRng) -> Vec<f32> {
    let mut occupied = vec![false; size * size];
    let mut depth = vec![0u32; size * size];
    let mut max_depth: u32 = 0;

    // seed center
    let center = (size / 2) * size + (size / 2);
    occupied[center] = true;
    depth[center] = 0;

    let max_steps = size * 8;

    for _ in 0..walkers {
        let mut x = rng.gen_range(0..size);
        let mut y = rng.gen_range(0..size);

        if occupied[y * size + x] {
            continue;
        }

        for _ in 0..max_steps {
            let mut neighbor_depth = None;
            for &(dx, dy) in &[(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                    let ni = ny as usize * size + nx as usize;
                    if occupied[ni] {
                        neighbor_depth = Some(depth[ni]);
                        break;
                    }
                }
            }

            if let Some(nd) = neighbor_depth {
                let idx = y * size + x;
                occupied[idx] = true;
                depth[idx] = nd + 1;
                if nd + 1 > max_depth {
                    max_depth = nd + 1;
                }
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

    // root = tallest (max_depth), tips = shortest (0)
    let md = max_depth.max(1) as f32;
    occupied
        .iter()
        .zip(depth.iter())
        .map(|(&occ, &d)| {
            if occ {
                (md - d as f32) / md
            } else {
                0.0
            }
        })
        .collect()
}

fn sample(data: &[f32], size: usize, x: i32, y: i32) -> f32 {
    let cx = x.clamp(0, size as i32 - 1) as usize;
    let cy = y.clamp(0, size as i32 - 1) as usize;
    data[cy * size + cx]
}

fn blur(data: &mut [f32], size: usize, passes: usize) {
    for _ in 0..passes {
        let prev = data.to_owned();
        for y in 0..size {
            for x in 0..size {
                let ix = x as i32;
                let iy = y as i32;
                let sum = sample(&prev, size, ix - 1, iy - 1)
                    + sample(&prev, size, ix, iy - 1)
                    + sample(&prev, size, ix + 1, iy - 1)
                    + sample(&prev, size, ix - 1, iy)
                    + sample(&prev, size, ix, iy)
                    + sample(&prev, size, ix + 1, iy)
                    + sample(&prev, size, ix - 1, iy + 1)
                    + sample(&prev, size, ix, iy + 1)
                    + sample(&prev, size, ix + 1, iy + 1);
                data[y * size + x] = sum / 9.0;
            }
        }
    }
}

fn upscale(src: &[f32], src_size: usize, dst_size: usize) -> Vec<f32> {
    let mut dst = vec![0.0; dst_size * dst_size];
    let scale = src_size as f32 / dst_size as f32;

    for dy in 0..dst_size {
        for dx in 0..dst_size {
            let sx = dx as f32 * scale;
            let sy = dy as f32 * scale;

            let x0 = (sx.floor() as usize).min(src_size - 1);
            let y0 = (sy.floor() as usize).min(src_size - 1);
            let x1 = (x0 + 1).min(src_size - 1);
            let y1 = (y0 + 1).min(src_size - 1);

            let fx = sx - x0 as f32;
            let fy = sy - y0 as f32;

            dst[dy * dst_size + dx] = src[y0 * src_size + x0] * (1.0 - fx) * (1.0 - fy)
                + src[y0 * src_size + x1] * fx * (1.0 - fy)
                + src[y1 * src_size + x0] * (1.0 - fx) * fy
                + src[y1 * src_size + x1] * fx * fy;
        }
    }

    dst
}

pub fn generate(seed: u32, walkers: usize, blur_passes: usize) -> Heightmap {
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let dla_size = 64;
    let grid = grow_dla(dla_size, walkers, &mut rng);

    let mut full = upscale(&grid, dla_size, GRID_SIZE);
    blur(&mut full, GRID_SIZE, blur_passes);

    let mut hmap = Heightmap::new(GRID_SIZE);
    hmap.data = full;
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
    fn blur_reduces_roughness() {
        let size = 32;
        let mut raw = vec![0.0; size * size];
        raw[(size / 2) * size + (size / 2)] = 1.0;
        raw[(size / 2 + 1) * size + (size / 2)] = 1.0;

        let mut blurred = raw.clone();
        blur(&mut blurred, size, 5);

        let roughness = |data: &[f32]| -> f32 {
            let mut total = 0.0;
            for y in 1..size {
                for x in 1..size {
                    let dx = data[y * size + x] - data[y * size + (x - 1)];
                    let dy = data[y * size + x] - data[(y - 1) * size + x];
                    total += dx.abs() + dy.abs();
                }
            }
            total
        };

        assert!(roughness(&blurred) < roughness(&raw));
    }

    #[test]
    fn root_is_tallest() {
        let mut rng = StdRng::seed_from_u64(42);
        let grid = grow_dla(32, 500, &mut rng);
        let center = (32 / 2) * 32 + (32 / 2);
        assert!(
            (grid[center] - 1.0).abs() < 1e-5,
            "root should be tallest (1.0), got {}",
            grid[center]
        );
    }

    #[test]
    fn tips_are_shorter_than_root() {
        let mut rng = StdRng::seed_from_u64(42);
        let grid = grow_dla(32, 500, &mut rng);
        let center = (32 / 2) * 32 + (32 / 2);
        let root_height = grid[center];
        let occupied_heights: Vec<f32> = grid.iter().copied().filter(|&v| v > 0.0).collect();
        let min_occupied = occupied_heights.iter().copied().fold(f32::INFINITY, f32::min);
        assert!(
            min_occupied < root_height,
            "tips ({}) should be shorter than root ({})",
            min_occupied, root_height
        );
    }

    #[test]
    fn no_edge_artifacts() {
        let hmap = generate(42, 2000, 3);
        let s = hmap.size;
        for x in 1..s - 1 {
            let edge = hmap.get(x, 0);
            let inner = hmap.get(x, 1);
            assert!(
                (edge - inner).abs() < 0.3,
                "edge artifact at ({}, 0): edge={}, inner={}",
                x, edge, inner
            );
        }
    }
}
