// G - OpenGL - Layer
// Desmond Germans, 2020

use crate::Framebuffer;

pub struct Layer {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub framebuffer: Framebuffer,
}

impl Layer {
    pub fn new(x: i32,y: i32,width: u32,height: u32) -> Option<Layer> {
        let framebuffer = match Framebuffer::new(width,height) {
            Some(framebuffer) => framebuffer,
            None => { return None; },
        };
        Some(Layer {
            x: x,
            y: y,
            width: width,
            height: height,
            framebuffer: framebuffer,
        })
    }

    pub fn bind(&self) {
        self.framebuffer.bind();
    }

    pub fn unbind(&self) {
        self.framebuffer.unbind();
    }
}
