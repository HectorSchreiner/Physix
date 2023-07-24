
use crate::core::physix::*;

pub struct Particle {
    position: Vector3,
    velocity: Vector3,
    acceleration: Vector3,
    damping: real,
    inverse_mass: real,
    force_accumulated: Vector3,
}

impl Particle {
    pub fn get_inverse_mass(self) -> real {
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
    fn update_force(particle: &Particle, duration: real) { todo!() }
}