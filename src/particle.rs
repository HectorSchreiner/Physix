use crate::{
    core::{physix::*, rk4},
    renderer::{Draw, Rectangle},
};

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

    fn integrate_rk4(&mut self, duration: real) {
        assert!(duration > 0.0);

        // get position from velocity
        self.position.add_scaled_vector(self.velocity, duration);

        // get velocity from acceleration
        self.velocity = rk4(self.acceleration, duration);

        // F = m*a -> a = F/m får acceleration ud fra den akkumulerede kraft
        self.acceleration += self.force_accumulated.mul(self.get_inverse_mass());
    }
}

pub trait ParticleForceGenerator {
    fn update_force(&self, particle: &mut Particle, duration: real) {}
}

pub struct ParticleForceRegistration<'a, T: ParticleForceGenerator> {
    pub particle: &'a mut Particle,
    pub particle_force_generator: T,
}

pub struct ParticleForceRegistry<'a, T: ParticleForceGenerator> {
    pub registrations: Vec<ParticleForceRegistration<'a, T>>,
}

impl<'a, T> ParticleForceRegistry<'a, T>
where
    T: ParticleForceGenerator,
{
    pub fn update_forces(&mut self, duration: real) {
        self.registrations.iter_mut().for_each(|reg| {
            reg.particle_force_generator
                .update_force(reg.particle, duration)
        });
    }

    pub fn add_generator_and_particle(
        &mut self,
        particle: &mut Particle,
        particle_force_generator: T,
    ) {
        let new_registration = ParticleForceRegistration {
            particle,
            particle_force_generator,
        };
        self.registrations.push(ParticleForceRegistration {
            particle,
            particle_force_generator: new_registration,
        });
    }
}

pub struct ParticleGravityForceGenerator {
    pub gravity: Vector3,
}

impl ParticleForceGenerator for ParticleGravityForceGenerator {
    fn update_force(&self, particle: &mut Particle, duration: real) {
        // hvis masse er uendelig returner
        if particle.get_inverse_mass() >= 0.0 {
            return;
        }

        // apply force scaled with mass to particle
        particle.add_force(&Vector3::mul(self.gravity, particle.get_inverse_mass()));
    }
}
