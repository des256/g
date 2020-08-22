// G Sprite Editor - Document
// by Desmond Germans, 2020

use e::*;
use std::{
    cell::Cell,
    rc::Rc,
};

pub struct Layer {
    pub(crate) texture: gpu::Texture2D::<e::pixel::ARGB8>,
    pub(crate) blend_mode: gpu::BlendMode,
}

impl Layer {
    pub fn new(graphics: &Rc<gpu::Graphics>,size: Vec2<usize>) -> Result<Layer,SystemError> {
        let texture = gpu::Texture2D::<e::pixel::ARGB8>::new(graphics,size).expect("Unable to create layer texture.");
        texture.set_filter(gpu::TextureFilter::Nearest);
        Ok(Layer {
            texture: texture,
            blend_mode: gpu::BlendMode::Over,
        })
    }
}

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

pub struct Document {
    pub(crate) layers: Vec<Rc<Layer>>,
    pub(crate) selection: Selection,
    pub(crate) offset: Cell<Vec2<f32>>,
    pub(crate) scale: Cell<Vec2<f32>>,
    pub(crate) background_grid_size: Cell<Vec2<i32>>,
}

impl Document {
    pub fn new(graphics: &Rc<gpu::Graphics>,size: Vec2<usize>) -> Document {
        Document {
            layers: vec![Rc::new(Layer::new(graphics,size).expect("Unable to create layer."))],
            selection: Selection::new(graphics),
            offset: Cell::new(vec2!(0.0,0.0)),
            scale: Cell::new(vec2!(1.0,1.0)),
            background_grid_size: Cell::new(vec2!(16,16)),
        }
    }
}
