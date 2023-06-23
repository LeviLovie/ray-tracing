#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

mod window;
mod config;

const VERSION: &str = "0.0.2";

fn main() {
    println!("Booting v{}", VERSION);

    let mut sphere = config::Object::new(config::TYPE_SPHERE, config::Transform::new(0.0, 0.0, 1.0));
    sphere.info();

    let mut window = window::WindowClass::new("Ray Casting");
    window.info();
    window.init_drawing();
}

