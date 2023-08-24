mod core;
mod particle;
mod renderer;

pub use minifb;
use minifb::*;
use renderer::*;

use crate::core::*;
use crate::particle::*;

// Tested on Windows with msvc compiler toolchain
pub fn main() {
    let mut renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut timer: core::physix::real = 0.0;

    let mut window = Window::new("Physix", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut particle_gravity_fg = ParticleGravityForceGenerator {
        gravity: physix::Vector3::new(0.0, -9.81, 0.0),
    };

    let mut particle_registry: ParticleForceRegistry<'_, ParticleGravityForceGenerator> =
        ParticleForceRegistry {
            registrations: vec![],
        };

    for p in 0..10 {
        let mut particle = Particle::default(physix::Vector3::new((p * 100) as f32, 200.0, 0.0));

        let mut particle_gravity_registration = ParticleForceRegistration {
            particle: &mut particle,
            particle_force_generator: particle_gravity_fg,
        };

        particle_registry
        .registrations
        .push(particle_gravity_registration);
    }
    
    // add gravity

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    timer += 0.1;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        (&mut renderer, &window);
        renderer.clear(Color::BLACK); //Clear screen

        particle_registry.update_forces(timer);
        particle_registry
            .registrations
            .iter_mut()
            .for_each(|particle_registration| {
                draw_particle(particle_registration.particle, &mut renderer)
            })
    }
}

fn draw_particle<'a>(particle: &'a mut Particle, renderer: &mut Renderer) {
    let rect = Rectangle::new(
        10,
        10,
        (particle.position.x as u32, particle.position.y as u32),
    );
    rect.draw(renderer, Color::WHITE);
}
