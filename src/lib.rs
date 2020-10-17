// G
// Desmond Germans, 2020

pub use base::*;
pub mod e {
    pub use platform::*;
    pub use gpu::*;
    pub use imageformats::*;
}

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