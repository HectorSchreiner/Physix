mod core;
mod particle;
use crate::core::physix;

fn main() {
    println!("Hello, world!");
    let a = physix::Vector3::new(1.0, 0.0, 0.0);
    let b = physix::Vector3::new(0.0, 1.0, 0.0);
    let c = a.cross_product(b);

    println!("cross {:?} invert: {:?}", c, c.invert());
}
