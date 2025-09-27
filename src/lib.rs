use bevy::app::{App, Plugin};

mod mesh;
pub use mesh::MeshBuilder;

pub struct WgslParticlePlugin;
impl Plugin for WgslParticlePlugin {
    fn build(&self, _app: &mut App) {}
}
