use crate::{
    core::{physix::*, rk4},
    renderer::{Draw, Rectangle, Color},
};

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
    damping: real,
    inverse_mass: real,
    pub force_accumulated: Vector3,
}

impl Particle {
    pub fn default(position: Vector3) -> Self {
        Self {
            position,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            damping: 1.0,
            inverse_mass: 1.0,
            force_accumulated: Vector3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn new(
        self,
        position: Vector3,
        velocity: Vector3,
        acceleration: Vector3,
        damping: real,
        inverse_mass: real,
        force_accumulated: Vector3,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            damping,
            inverse_mass,
            force_accumulated,
        }
    }
    pub fn get_inverse_mass(&self) -> real {
        self.inverse_mass
    }

    pub fn set_inverse_mass(&mut self, mass: real) {
        self.inverse_mass = mass;
    }

    fn clear_accumulator(&mut self) {
        self.force_accumulated.clear();
    }

    fn add_force(&mut self, force: &Vector3) {
        self.force_accumulated += *force;
    }
}

pub struct ParticleSize {
    start: f32,
    end: f32,
}

pub struct ParticleVelocity {
    start: Vector3,
    end: Vector3,
}

pub struct ParticleColor {
    start: Color,
    end: Color,
}
