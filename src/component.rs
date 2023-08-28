use crate::particle::*;

pub struct Component;

#[derive(Component, Debug, Clone)]
pub struct ParticleSpawner {
    rate: f32,
    ammount_per_burst: usize,
    particle_lifetime: f32,
    position_variance: f32,
    particle_size: Option<ParticleSize>,
    particle_color: Option<ParticleColor>,
    particle_velocity: Option<ParticleVelocity>,
    timer: Timer,
}
