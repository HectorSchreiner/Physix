
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
    pub fn get_inverse_mass(&self) -> real {
        self.inverse_mass
    }

    pub fn set_inverse_mass(&mut self, mass: real) {
        self.inverse_mass = mass;
    }  

    fn integrate(&mut self, duration: real) {
       assert!(duration > 0.0);

       // update linear position
        self.position.add_scaled_vector(self.velocity, duration);

       // find acceleration from force
        let mut resulting_acceleration = self.acceleration;
        resulting_acceleration.add_scaled_vector(self.force_accumulated, self.inverse_mass);

       // update linear velocity from acceleration
        self.velocity.add_scaled_vector(resulting_acceleration, duration);

       // drag
       self.velocity.mul_assign(self.damping.powf(duration));

       // clear forces
       self.clear_accumulator();
    }

    fn clear_accumulator(&mut self) {
        self.force_accumulated.clear();
    } 

    fn add_force(&mut self, force: &Vector3) {
        self.force_accumulated += *force;
    }
}
pub trait ParticleForceGenerator {
    fn update_force(&self, particle: &Particle, duration: real) { todo!() }
}

struct ParticleForceRegistration<'a> {
    particle: &'a Particle,
    fg: &'a dyn ParticleForceGenerator,
}

struct ParticleForceRegistry<'a> {
    registrations: Vec<ParticleForceRegistration<'a>>,
}

impl ParticleForceRegistry<'_> {
    fn update_forces(&mut self, duration: real) {
        self.registrations.
        iter().
        for_each(|i| {
            i.fg.update_force(i.particle, duration)
        });
    }
}


struct ParticleGravity;

impl ParticleForceGenerator for ParticleGravity {
    fn update_force(&self, particle: &Particle, duration: real) {
        // hvis masse er uendelig returner
        if particle.get_inverse_mass() >= 0.0 { return; }

        // apply force scaled with mass to particle
        particle.add_force(gravity * 1/particle.get_inverse_mass())

    }
}