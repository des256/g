// Kvasir
// Desmond Germans, 2020

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum WindowConfig {
    Standard,  // 640x360
    High,  // 1280x720
}

pub enum FramebufferConfig {
    Standard,  // 640x360
    Low,  // 320x180
}

pub struct VideoConfig {
    pub window: WindowConfig,
    pub framebuffer: FramebufferConfig,
}

pub enum Button {
    Left,
    Middle,
    Right,
}

impl Display for Button {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            Button::Left => { write!(f,"left") },
            Button::Middle => { write!(f,"middle") },
            Button::Right => { write!(f,"right") },
        }
    }
}

pub enum Wheel {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Wheel {
    fn fmt(&self,f: &mut Formatter) -> Result {
        match self {
            Wheel::Up => { write!(f,"up") },
            Wheel::Down => { write!(f,"down") },
            Wheel::Left => { write!(f,"left") },
            Wheel::Right => { write!(f,"right") },
        }
    }
}

pub enum Event {
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(i32,i32,Button),
    MouseRelease(i32,i32,Button),
    MouseWheel(Wheel),
    MouseMove(i32,i32),
    Paint(i32,i32,u32,u32),
    Resize(u32,u32),
    Close,
}

pub enum VideoError {
    Generic,
}

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::*;
#[cfg(target_os="linux")]
mod opengl45;
#[cfg(target_os="linux")]
pub use opengl45::*;
#[cfg(target_os="linux")]
mod alsa;
#[cfg(target_os="linux")]
pub use ::alsa::*;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::*;
#[cfg(target_os="windows")]
mod opengl45;
#[cfg(target_os="windows")]
pub use opengl45::*;
#[cfg(target_os="windows")]
mod directsound;
#[cfg(target_os="windows")]
pub use directsound::*;

#[cfg(target_os="macos")]
mod macos;
#[cfg(target_os="macos")]
pub use macos::*;
#[cfg(target_os="macos")]
mod opengl45;
#[cfg(target_os="macos")]
pub use opengl45::*;
// TODO: audio for MacOS

#[cfg(target_os="android")]
mod android;
#[cfg(target_os="android")]
pub use android::*;
#[cfg(target_os="android")]
mod gles20;
#[cfg(target_os="android")]
pub use gles20::*;
// TODO: audio for Android

#[cfg(target_os="ios")]
mod ios;
#[cfg(target_os="ios")]
pub use ios::*;
#[cfg(target_os="ios")]
mod gles20;
#[cfg(target_os="ios")]
pub use gles20::*;
// TODO: audio for iOS

#[cfg(target_arch="wasm32")]
mod web;
#[cfg(target_arch="wasm32")]
pub use web::*;
#[cfg(target_arch="wasm32")]
mod webgl1;
#[cfg(target_arch="wasm32")]
pub use webgl1::*;
#[cfg(target_arch="wasm32")]
mod webaudio;
#[cfg(target_arch="wasm32")]
pub use webaudio::*;

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
