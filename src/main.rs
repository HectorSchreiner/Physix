mod core;
mod particle;
mod renderer;
pub use minifb;
use minifb::*;
use renderer::*;

use crate::core::*;

// Tested on Windows with msvc compiler toolchain
pub fn main() {
    let mut renderer = Renderer {
        buffer: vec![0; WIDTH * HEIGHT],
    };

    let mut window = Window::new(
        "Physix",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&renderer.buffer, WIDTH, HEIGHT)
            .unwrap();
        (&mut renderer, &window);

        renderer.clear(Color::BLACK); //Clear screen
        Rectangle::new(20, 20, (20, 20)).draw(&mut renderer, Color::WHITE);
    }
}
