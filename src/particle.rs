use crate::core::physix::*;

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

pub trait ParticleForceGenerator {
    fn update_force(&self, particle: &mut Particle, duration: real) {}
}

pub struct ParticleForceRegistration<'a> {
    particle: &'a mut Particle,
    force_generator: &'a dyn ParticleForceGenerator,
}

pub struct ParticleForceRegistry<'a> {
    registrations: Vec<ParticleForceRegistration<'a>>,
}

impl ParticleForceRegistry<'_> {
    fn update_forces(&mut self, duration: real) {
        self.registrations
            .iter_mut()
            .for_each(|reg| reg.force_generator.update_force(reg.particle, duration));
    }
}

struct ParticleGravity {
    gravity: Vector3,
}

impl ParticleForceGenerator for ParticleGravity {
    fn update_force(&self, particle: &mut Particle, duration: real) {
        // hvis masse er uendelig returner
        if particle.get_inverse_mass() >= 0.0 {
            return;
        }

        // apply force scaled with mass to particle
        particle.add_force(&Vector3::mul(self.gravity, particle.get_inverse_mass()));
    }
}
