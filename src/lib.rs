// G
// Desmond Germans, 2020

use e::*;

pub trait Layer {
    fn framebuffer(&self) -> &gpu::Framebuffer;
    fn render(&self);
}

mod staticlayer;
pub use staticlayer::*;

mod maplayer;
pub use maplayer::*;

mod engine;
pub use engine::*;