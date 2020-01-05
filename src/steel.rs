// Steel Engine main
// by Desmond Germans, 2020

use e::*;
use crate::*;

/// The game engine.
pub struct Steel {
    video: e::Video,
    wid: usize,
    audio: e::Audio,
}

impl Steel {
    /// Create new game engine.
    fn new(title: &str) -> Steel {
        let mut video = e::Video::new();
        let wid = video.create_app_window(e::isize_r::new(100,100,1920,1080),title);
        let audio = e::Audio::new();
        Steel {
            video: video,
            wid: wid,
            audio: audio,
        }
    }
}
