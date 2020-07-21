// G - OpenGL - Layer
// Desmond Germans, 2020

use crate::*;

pub struct Layer {
    pub r: Rect<isize>,
    pub framebuffer: Framebuffer,
}

impl Layer {
    pub fn new(engine: &Engine,r: Rect<isize>) -> Result<Layer,EngineError> {
        let framebuffer = match engine.system.create_framebuffer(vec2!(r.s.x as usize,r.s.y as usize)) {
            Ok(framebuffer) => framebuffer,
            Err(_) => { return Err(EngineError::Generic); },
        };
        Ok(Layer {
            r: r,
            framebuffer: framebuffer,
        })
    }
}

impl<'a> Engine<'a> {
    pub fn bind_layer(&self,layer: &Layer) {
        self.system.bind_framebuffer(&layer.framebuffer);
    }

    pub fn unbind_layer(&self) {
        self.system.unbind_framebuffer();
    }
}
