// G - TGA
// Desmond Germans, 2020

use crate::Image;
use crate::Pixel;

pub fn test(_src: &[u8]) -> Option<(u32,u32)> {
    None
}

pub fn decode<T: Pixel>(_src: &[u8]) -> Option<Image<T>> {
    None
}

pub fn encode<T: Pixel>(_image: &Image<T>) -> Option<Vec<u8>> {
    None
}
