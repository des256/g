// G - OpenGL - Layer
// Desmond Germans, 2020

use crate::*;
use std::rc::Rc;

/// Game engine compositing layer.
pub struct Layer {
    pub(crate) r: Rect<isize>,
    pub(crate) framebuffer: Rc<gpu::Framebuffer>,
}

impl Layer {
    /// Create new layer.
    /// # Arguments
    /// * `engine` - Engine to create layer for.
    /// * `r` - rectangle for this layer.
    pub fn new(engine: &Engine,r: Rect<isize>) -> Result<Layer,EngineError> {
        let framebuffer = Rc::new(match gpu::Framebuffer::new(&engine.graphics,vec2!(r.s.x as usize,r.s.y as usize)) {
            Ok(framebuffer) => framebuffer,
            Err(_) => { return Err(EngineError::Generic); },
        });
        Ok(Layer {
            r: r,
            framebuffer: framebuffer,
        })
    }
}
