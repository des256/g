// G - Video
// Desmond Germans, 2020

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Copy,Clone)]
pub struct WindowConfig {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy,Clone)]
pub struct FramebufferConfig {
    pub width: usize,
    pub height: usize,
}

#[derive(Copy,Clone)]
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

