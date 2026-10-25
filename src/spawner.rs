use std::{marker::PhantomData, time::Duration};

use bevy::{
    app::{App, Plugin, Update},
    asset::Assets,
    ecs::{
        component::Component,
        name::Name,
        system::{Commands, Query, Res, ResMut},
    },
    log::info,
    reflect::Reflect,
    render::storage::ShaderBuffer,
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    utils::default,
};

use crate::{Effect, Particle, ParticleAssets};

pub struct ParticleSpawnerPlugin<E: Effect> {
    pub _marker: PhantomData<E>,
}
impl<E: Effect> Plugin for ParticleSpawnerPlugin<E> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ParticleSpawner::<E>::update);
    }
}
impl<E: Effect> Default for ParticleSpawnerPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

/// Particle spawner component. Spawn this to add new live particles to the effect.
#[derive(Component, Clone, Debug, Reflect)]
#[require(Name::new("ParticleSpawner"))]
pub struct ParticleSpawner<E: Effect> {
    pub n: usize,
    pub timer: Timer,
    pub _marker: PhantomData<E>,
}
impl<E: Effect> ParticleSpawner<E> {
    pub fn new(n: usize, duration: Duration) -> Self {
        let mut timer = Timer::new(duration, TimerMode::Repeating);
        timer.almost_finish();
        Self {
            n,
            timer,
            _marker: PhantomData,
        }
    }
    pub fn update(
        commands: Commands,
        mut query: Query<(&mut Self, &Transform)>,
        time: Res<Time>,
        assets: Res<ParticleAssets<E>>,
        mut buffers: ResMut<Assets<ShaderBuffer>>,
    ) {
        let _ = commands;
        for (mut spawner, transform) in query.iter_mut() {
            if !spawner.timer.tick(time.delta()).is_finished() {
                continue;
            }
            if spawner.n == 0 {
                continue;
            }

            if let Some(mut buf) = buffers.get_mut(&assets.buffer) {
                info!("Spawn!");
                // TODO: How do I update a slice of buffer?
                buf.clear();
                buf.extend(spawner.spawn_particles(transform));
                // Temp: fill remainder will inactive particles.
                buf.extend(vec![
                    Particle::default();
                    E::particles_per_instance() * (E::max_instances() - 1)
                ]);
                spawner.n -= 1;
            }
        }
    }
    fn spawn_particles(&self, transform: &Transform) -> Vec<Particle> {
        let mut particles = Vec::with_capacity(self.n);
        for _ in 0..E::particles_per_instance() {
            particles.push(Particle {
                position: transform.translation,
                initialized: false as u32,
                ..default()
            });
        }
        return particles;
    }
}
impl<E: Effect> Default for ParticleSpawner<E> {
    fn default() -> Self {
        Self::new(1, Duration::from_secs(1))
    }
}
