// G Sprite Editor - Layer
// by Desmond Germans, 2020

use e::*;
use std::rc::Rc;

pub struct Layer {
    pub(crate) texture: gpu::Texture2D::<e::pixel::ARGB8>,
    pub(crate) blend_mode: gpu::BlendMode,
}

impl Layer {
    pub fn new(graphics: &Rc<gpu::Graphics>) -> Result<Layer,SystemError> {
        Ok(Layer {
            texture: gpu::Texture2D::<e::pixel::ARGB8>::new(graphics,vec2!(1024,1024)).expect("Unable to create layer texture."),
            blend_mode: gpu::BlendMode::Over,
        })
    }
}
