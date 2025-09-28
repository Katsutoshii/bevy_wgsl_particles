use bevy::asset::RenderAssetUsages;
use bevy::math::{UVec2, Vec2};
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

/// Utility struct for building a mesh.
#[derive(Default)]
pub struct MeshBuilder {
    pub positions: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

impl MeshBuilder {
    pub fn quad_indices(x: u32, y: u32, w: u32) -> [u32; 6] {
        let q = x + y * w;
        let i = q * 4;
        [i, i + 1, i + 2, i, i + 2, i + 3]
    }

    pub fn get_position(x: u32, y: u32, size: UVec2) -> [f32; 3] {
        (UVec2::new(x, y).as_vec2() / size.as_vec2() - Vec2::splat(0.5))
            .extend(0.0)
            .to_array()
    }

    pub fn quad_positions(x: u32, y: u32, size: UVec2) -> [[f32; 3]; 4] {
        [
            Self::get_position(x, y, size),
            Self::get_position(x + 1, y, size),
            Self::get_position(x + 1, y + 1, size),
            Self::get_position(x, y + 1, size),
        ]
    }

    pub fn quad_uvs() -> [[f32; 2]; 4] {
        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]]
    }

    /// Compute a grid mesh of quads according to size.
    pub fn grid(size: UVec2) -> Self {
        let num_quads = size.x as usize * size.y as usize;
        let mut builder = Self {
            positions: Vec::with_capacity(num_quads * 4),
            uvs: Vec::with_capacity(num_quads * 4),
            indices: Vec::with_capacity(num_quads * 6),
        };
        for y in 0..size.y {
            for x in 0..size.x {
                builder.positions.extend(Self::quad_positions(x, y, size));
                builder.uvs.extend(Self::quad_uvs());
                builder.indices.extend(Self::quad_indices(x, y, size.x))
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
