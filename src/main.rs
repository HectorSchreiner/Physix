mod core;
use crate::core::physix;

fn main() {
    println!("Hello, world!");
    let a = physix::Vector3 {x: 1.0, y: 0.0, z: 0.0};
    let b = physix::Vector3 {x: 0.0, y: 1.0, z: 1.0};
    let c = a - b;

    println!("{:?}", c);
}
