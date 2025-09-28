#import bevy_wgsl_particles::particle::{Particle, ComputeInput, particles};

@group(0) @binding(1) var<uniform> dt: f32;

// Update the position of each particle.
@compute @workgroup_size(16, 1, 1)
fn update(in: ComputeInput) {
    let velocity = vec3<f32>(0.1, 0.0, 0.0);
    let i = in.id.x;
    if i < 4 {
        particles[i].position += velocity * dt;
    }
}
