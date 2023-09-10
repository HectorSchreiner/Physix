use std::{
    ops::{Deref, DerefMut},
    rc::Rc, cell::RefCell,
};

use crate::{
    core::{physix::*, rk4},
    renderer::{Color, Draw, Rectangle},
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

    pub fn integrate(&mut self, duration: real) {
        assert!(duration > 0.0);

        // work out acceleration from force
        self.position.add_scaled_vector(self.velocity, duration);
        let resulting_acceleration = self.acceleration;

        // update velocity from force
        self.velocity
            .add_scaled_vector(self.force_accumulated, self.get_inverse_mass());

        // drag
        self.velocity.mul_assign(self.damping.powf(duration));

        // clear forces
        self.clear_accumulator();
    }
}

pub struct ParticleRegistry<'a> {
    pub registations: Vec<&'a mut ParticleRegistration<'a>>,
}

pub struct ParticleRegistration<'a> {
    pub particle: Rc<RefCell<&'a mut Particle>>,
    pub force_generator: &'a dyn ParticleForceGenerator,
}

pub trait ParticleForceGenerator {
    fn update_force(&self, particle: Rc<RefCell<&mut Particle>>, duration: real) {}
}

pub struct ParticleGravityForceGenerator;

impl ParticleForceGenerator for ParticleGravityForceGenerator {
    fn update_force(&self, particle: Rc<RefCell<&mut Particle>>, duration: real) {
        let gravity = Vector3::new(0.0, 9.81, 0.0);
        let mut particle = particle.borrow_mut();
        particle.clone().add_force(&gravity);
    }
}

impl<'a> ParticleRegistry<'a> {
    pub fn new() -> Self {
        ParticleRegistry {
            registations: Vec::new(),
        }
    }

    pub fn add_registration(&mut self, registration: &'a mut ParticleRegistration<'a>) {
        self.registations.push(registration);
    }

    // removes pair from registry, if pair doesnt exist then ignore...
    pub fn remove_registration(&mut self, registration: &'a mut ParticleRegistration<'a>) {
        todo!()
    }

    pub fn clear_registry(&mut self) {
        self.registations.clear()
    }
    pub fn update_registry_forces(&mut self, duration: real) {
        self.registations.iter_mut().for_each(|reg| {
            reg.force_generator
                .update_force(reg.particle.clone(), duration)
        });
    }
}
