use crate::window;
use rand::Rng;

pub fn ProcessPixel(SizeY: usize, SizeX: usize, PosY: usize, PosX: usize) -> window::Color {
    return window::Color::new(
        rand::thread_rng().gen_range(0..100) as f64 / 100.0,
        rand::thread_rng().gen_range(0..100) as f64 / 100.0,
        rand::thread_rng().gen_range(0..100) as f64 / 100.0,
    ) 
}
