use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

use super::Heightmap;

const HEIGHT_SCALE: f32 = 30.0;
const TERRAIN_SCALE: f32 = 1.0;

fn height_color(height: f32, slope: f32) -> [f32; 4] {
    // steep slopes get rocky gray regardless of height
    if slope > 0.6 {
        return [0.45, 0.43, 0.40, 1.0];
    }

    if height < 0.15 {
        // deep valleys - dark green
        [0.18, 0.32, 0.15, 1.0]
    } else if height < 0.35 {
        // lower slopes - green
        [0.25, 0.42, 0.18, 1.0]
    } else if height < 0.55 {
        // mid elevation - lighter green fading to brown
        let t = (height - 0.35) / 0.2;
        [
            0.25 + t * 0.25,
            0.42 - t * 0.12,
            0.18 + t * 0.07,
            1.0,
        ]
    } else if height < 0.75 {
        // high slopes - gray rock
        let t = (height - 0.55) / 0.2;
        [
            0.50 + t * 0.15,
            0.30 + t * 0.18,
            0.25 + t * 0.20,
            1.0,
        ]
    } else {
        // peaks - snow white
        let t = (height - 0.75) / 0.25;
        [
            0.65 + t * 0.30,
            0.48 + t * 0.47,
            0.45 + t * 0.50,
            1.0,
        ]
    }
}

pub fn build_terrain_mesh(heightmap: &Heightmap) -> Mesh {
    let size = heightmap.size;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(size * size);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(size * size);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(size * size);
    let mut indices: Vec<u32> = Vec::with_capacity((size - 1) * (size - 1) * 6);

    let half = (size as f32 * TERRAIN_SCALE) / 2.0;

    for y in 0..size {
        for x in 0..size {
            let h = heightmap.get(x, y);
            let px = x as f32 * TERRAIN_SCALE - half;
            let py = h * HEIGHT_SCALE;
            let pz = y as f32 * TERRAIN_SCALE - half;
            positions.push([px, py, pz]);
        }
    }

    // compute normals from finite differences
    for y in 0..size {
        for x in 0..size {
            let hx0 = if x > 0 { heightmap.get(x - 1, y) } else { heightmap.get(x, y) };
            let hx1 = if x < size - 1 { heightmap.get(x + 1, y) } else { heightmap.get(x, y) };
            let hy0 = if y > 0 { heightmap.get(x, y - 1) } else { heightmap.get(x, y) };
            let hy1 = if y < size - 1 { heightmap.get(x, y + 1) } else { heightmap.get(x, y) };

            let dx = (hx1 - hx0) * HEIGHT_SCALE;
            let dy = (hy1 - hy0) * HEIGHT_SCALE;
            let n = Vec3::new(-dx, 2.0 * TERRAIN_SCALE, -dy).normalize();
            normals.push([n.x, n.y, n.z]);

            let slope = 1.0 - n.y;
            colors.push(height_color(heightmap.get(x, y), slope));
        }
    }

    for y in 0..(size - 1) as u32 {
        for x in 0..(size - 1) as u32 {
            let tl = y * size as u32 + x;
            let tr = tl + 1;
            let bl = tl + size as u32;
            let br = bl + 1;
            indices.extend_from_slice(&[tl, bl, tr, tr, bl, br]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terrain::Heightmap;

    #[test]
    fn mesh_has_correct_vertex_count() {
        let mut hmap = Heightmap::new(16);
        for v in &mut hmap.data {
            *v = 0.5;
        }
        let mesh = build_terrain_mesh(&hmap);
        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        assert_eq!(positions.len(), 16 * 16);
    }

    #[test]
    fn mesh_has_correct_index_count() {
        let hmap = Heightmap::new(16);
        let mesh = build_terrain_mesh(&hmap);
        let idx_count = match mesh.indices().unwrap() {
            Indices::U32(v) => v.len(),
            _ => panic!("expected u32 indices"),
        };
        assert_eq!(idx_count, 15 * 15 * 6);
    }

    #[test]
    fn flat_terrain_has_upward_normals() {
        let mut hmap = Heightmap::new(8);
        for v in &mut hmap.data {
            *v = 0.5;
        }
        let mesh = build_terrain_mesh(&hmap);
        if let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(normals)) =
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
        {
            for n in normals {
                assert!((n[1] - 1.0).abs() < 1e-3, "expected y-up normal, got {:?}", n);
            }
        }
    }
}
