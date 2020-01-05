// engine
// by Desmond Germans, 2020

use e::*;
use crate::*;

/// The game engine.
pub struct Engine {
    video: e::Video,
    wid: usize,
    audio: e::Audio,
}

impl Engine {
    /// Create new game engine.
    fn new(title: &str) -> Engine {
        let video = e::Video::new();
        let wid = video.create_app_window(e::isize_r::new(100,100,1920,1080),title);
        let audio = e::Audio::new();
        Engine {
            video: video,
            wid: wid,
            audio: audio,
        }
    }
}
