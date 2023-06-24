use crate::window;
pub mod pixel;

use std::time::{Duration, Instant};

pub fn ProcessScreen(SizeY: usize, SizeX: usize) -> Vec<window::Color> {
    println!("Processing screen (it can take a few seconds)");
    let start = Instant::now();
    let mut result = vec![window::Color::new(0.0, 0.0, 0.0); SizeY*SizeX];
    for i in 0..SizeY*SizeX {
        result[i] = pixel::ProcessPixel(
            SizeY, SizeX,
            i / SizeY,
            i - ((i / SizeY) * SizeY),
        )
    }
    println!("Process Screen took {:?}", start.elapsed());
    return result;
}
