use crate::window;
pub mod pixel;

pub fn ProcessScreen(SizeY: usize, SizeX: usize) -> Vec<window::Color> {
    let mut result = vec![window::Color::new(0.0, 0.0, 0.0); SizeY*SizeX];
    for i in 0..SizeY*SizeX {
        result[i] = pixel::ProcessPixel(
            SizeY, SizeX,
            i / SizeY,
            i - ((i / SizeY) * SizeY),
        )
    }
    return result;
}
