// G - OpenGL - Layer
// Desmond Germans, 2020

use crate::Framebuffer;

pub struct Layer {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize,
    pub framebuffer: Framebuffer,
}

impl Layer {
    pub fn new(x: isize,y: isize,width: usize,height: usize) -> Option<Layer> {
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
