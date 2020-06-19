// Kvasir - System Interface - OpenGL - Layer
// Desmond Germans, 2020

pub struct Layer {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub framebuffer: crate::Framebuffer,
}

impl Layer {
    pub fn new(x: i32,y: i32,width: u32,height: u32) -> Layer {
        Layer {
            x: x,
            y: y,
            width: width,
            height: height,
            framebuffer: crate::Framebuffer::new(width,height),
        }
    }

    pub fn bind(&self) {
        self.framebuffer.bind();
    }

    pub fn unbind(&self) {
        self.framebuffer.unbind();
    }
}
