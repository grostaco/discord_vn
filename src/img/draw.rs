use image::{ImageBuffer, Rgba};

use super::size::Size;

pub struct Scene {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    sprite: Size,
    text: Size,
}
