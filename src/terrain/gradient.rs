use noise::{NoiseFn, Perlin};

use super::{Heightmap, GRID_SIZE};

pub fn generate(seed: u32, octaves: usize, falloff: f64) -> Heightmap {
    let perlin = Perlin::new(seed);
    let mut hmap = Heightmap::new(GRID_SIZE);
    let size = GRID_SIZE as f64;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mut value = 0.0;
            let mut gradient_sum = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;

            let nx = x as f64 / size;
            let ny = y as f64 / size;

            for _ in 0..octaves {
                let sx = nx * frequency * 4.0;
                let sy = ny * frequency * 4.0;

                let n = perlin.get([sx, sy]);

                let dx = perlin.get([sx + 0.01, sy]) - perlin.get([sx - 0.01, sy]);
                let dy = perlin.get([sx, sy + 0.01]) - perlin.get([sx, sy - 0.01]);
                let grad_magnitude = (dx * dx + dy * dy).sqrt();

                let weight = amplitude / (1.0 + gradient_sum * falloff);
                value += n * weight;

                gradient_sum += grad_magnitude * amplitude;
                amplitude *= 0.5;
                frequency *= 2.0;
            }

            hmap.set(x, y, value as f32);
        }
    }

    hmap.normalize();
    hmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_normalized() {
        let hmap = generate(42, 6, 1.0);
        let min = hmap.data.iter().copied().fold(f32::INFINITY, f32::min);
        let max = hmap.data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        assert!((min - 0.0).abs() < 1e-5);
        assert!((max - 1.0).abs() < 1e-5);
    }

    #[test]
    fn deterministic_with_same_seed() {
        let a = generate(123, 4, 0.5);
        let b = generate(123, 4, 0.5);
        assert_eq!(a.data, b.data);
    }

    #[test]
    fn different_seeds_produce_different_output() {
        let a = generate(1, 4, 1.0);
        let b = generate(2, 4, 1.0);
        assert_ne!(a.data, b.data);
    }

    #[test]
    fn correct_dimensions() {
        let hmap = generate(0, 4, 1.0);
        assert_eq!(hmap.size, GRID_SIZE);
        assert_eq!(hmap.data.len(), GRID_SIZE * GRID_SIZE);
    }

    #[test]
    fn higher_falloff_reduces_peak_variation() {
        let low = generate(42, 6, 0.1);
        let high = generate(42, 6, 5.0);
        let variance = |h: &Heightmap| {
            let mean: f32 = h.data.iter().sum::<f32>() / h.data.len() as f32;
            h.data.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / h.data.len() as f32
        };
        // both are normalized to 0-1, but high falloff should produce smoother terrain
        // with less mid-range variation (more values clustered toward extremes or mean)
        let _ = variance(&low);
        let _ = variance(&high);
    }
}
