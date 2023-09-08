mod core;
mod particle;
mod renderer;

use crate::core::physix::Vector3;
use std::rc::Rc;

pub use minifb;
use minifb::*;
use renderer::*;

use crate::core::*;
use crate::particle::*;

// Tested on Windows with msvc compiler toolchain
pub fn main() {
    let mut window = Window::new("Physix", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut particle_1 = Particle::default(Vector3::new(0.0, 0.0, 0.0));

    let mut gravity_force_generator = ParticleGravityForceGenerator {};

    let mut registry = ParticleRegistry {
        registations: vec![],
    };
    let registration_1 = ParticleRegistration {
        particle: Rc::from(&mut particle_1),
        force_generator: &mut gravity_force_generator,
    };
    let registration_2 = ParticleRegistration {
        particle: Rc::from(&mut particle_1),
        force_generator: &mut gravity_force_generator,
    };

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        (&mut renderer, &window);
    }
}
