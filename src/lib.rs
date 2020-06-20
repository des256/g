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

mod image;
pub use image::*;
