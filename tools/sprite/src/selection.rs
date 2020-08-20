// G Sprite Editor - Selection
// by Desmond Germans, 2020

use e::*;
use std::rc::Rc;

pub struct Selection {
    pub(crate) texture: gpu::Texture2D::<e::pixel::R8>,
}

impl Selection {
    pub fn new(graphics: &Rc<gpu::Graphics>) -> Selection {
        Selection {
            texture: gpu::Texture2D::<e::pixel::R8>::new(graphics,vec2!(1024,1024)).expect("Unable to create layer texture."),
        }
    }
}
