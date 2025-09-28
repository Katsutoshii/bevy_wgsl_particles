use bevy::asset::RenderAssetUsages;
use bevy::math::UVec2;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

/// Utility struct for building a mesh.
#[derive(Default)]
pub struct MeshBuilder {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshBuilder {
    /// Compute a grid mesh of quads according to size.
    pub fn grid(size: UVec2) -> Self {
        let num_quads = size.x as usize * size.y as usize;
        let mut builder = Self {
            positions: Vec::with_capacity(num_quads * 4),
            uvs: Vec::with_capacity(num_quads * 4),
            indices: Vec::with_capacity(num_quads * 6),
        };
        let w = size.x;
        for y in 0..size.y {
            for x in 0..size.x {
                let q = x + y * w;
                let i = q * 4;
                builder.positions.extend(
                    // Square quad, side length 1.
                    [
                        [-0.5, -0.5, -0.5],
                        [0.5, -0.5, -0.5],
                        [0.5, 0.5, -0.5],
                        [-0.5, 0.5, -0.5],
                    ],
                );
                builder.uvs.extend(
                    // [0,0] at top left, [1, 1] at bot right.
                    [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                );
                builder.indices.extend(
                    // Two triangles to make up the quad.
                    [i, i + 1, i + 2, i, i + 2, i + 3],
                )
            }
        }
        builder
    }

    /// Produce a mesh from the accumulated attributes.
    pub fn build(self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_indices(Indices::U32(self.indices))
        .with_computed_smooth_normals()
    }
}
