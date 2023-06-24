#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

mod window;
mod engine;

const VERSION: &str = "0.0.6";

fn main() {
    println!("Booting v{}", VERSION);

    let mut window = window::WindowClass::new("Ray Tracing");
    window.info();
    window.init_drawing();
}

