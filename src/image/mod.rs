// Kvasir - Image
// Desmond Germans, 2020

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Box<[u32]>,
}

impl Image {
    pub fn new(width: u32,height: u32) -> Image {
        Image {
            width: width,
            height: height,
            data: vec![0u32; (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn pixel(&self,x: i32,y: i32) -> u32 {
        self.data[(y * self.width as i32 + x) as usize]
    }

    pub fn set_pixel(&mut self,x: i32,y: i32,p: u32) {
        self.data[(y * self.width as i32 + x) as usize] = p;
    }
}

pub mod bmp;
pub mod png;
pub mod jpeg;
pub mod tga;
pub mod gif;
pub mod pbm;
pub mod tiff;
pub mod xbm;
pub mod webp;

#[allow(dead_code)]
pub fn test(src: &[u8]) -> Option<(u32,u32)> {
    if let Some(size) = bmp::test(src) {
        Some(size)
    }
    else if let Some(size) = png::test(src) {
        Some(size)
    }
    else if let Some(size) = jpeg::test(src) {
        Some(size)
    }
    else if let Some(size) = gif::test(src) {
        Some(size)
    }
    else if let Some(size) = tga::test(src) {
        Some(size)
    }
    else if let Some(size) = tiff::test(src) {
        Some(size)
    }
    else if let Some(size) = pbm::test(src) {
        Some(size)
    }
    else if let Some(size) = xbm::test(src) {
        Some(size)
    }
    else if let Some(size) = webp::test(src) {
        Some(size)
    }
    else {
        None
    }
}

#[allow(dead_code)]
pub fn decode(src: &[u8]) -> Option<Image> {
    if let Some(image) = bmp::decode(src) {
        Some(image)
    }
    else if let Some(image) = png::decode(src) {
        Some(image)
    }
    else if let Some(image) = jpeg::decode(src) {
        Some(image)
    }
    else if let Some(image) = gif::decode(src) {
        Some(image)
    }
    else if let Some(image) = tga::decode(src) {
        Some(image)
    }
    else if let Some(image) = tiff::decode(src) {
        Some(image)
    }
    else if let Some(image) = pbm::decode(src) {
        Some(image)
    }
    else if let Some(image) = xbm::decode(src) {
        Some(image)
    }
    else if let Some(image) = webp::decode(src) {
        Some(image)
    }
    else {
        None
    }
}
