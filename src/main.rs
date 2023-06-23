#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

mod window;

const VERSION: &str = "0.0.1";

fn main() {
    println!("Booting v{}", VERSION);
    let mut window = window::WindowClass::new(10, 10, "Ray Casting");
    window.info();
    window.init_drawing();
}

