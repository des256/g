// G Sprite Editor - Document
// by Desmond Germans, 2020

use e::*;
use std::rc::Rc;

use crate::layer::*;
use crate::selection::*;
use crate::grid::*;
use crate::pixelgrid::*;

pub struct Document {
    pub(crate) layers: Vec<Rc<Layer>>,
    pub(crate) selection: Selection,
    pub(crate) grid: Grid,
    pub(crate) pixel_grid: PixelGrid,
}

impl Document {
    pub fn new(graphics: &Rc<gpu::Graphics>) -> Document {
        Document {
            layers: vec![Rc::new(Layer::new(graphics).expect("Unable to create layer."))],
            selection: Selection::new(),
            grid: Grid::new(),
            pixel_grid: PixelGrid::new(),
        }
    }
}
