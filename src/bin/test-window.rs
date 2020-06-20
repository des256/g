// G - Window test
// Desmond Germans, 2020

use g::Video;
use g::VideoConfig;
use g::WindowConfig;
use g::FramebufferConfig;
use g::Event;

fn main() {
    let mut video = match Video::new(VideoConfig {
        window: WindowConfig::High,
        framebuffer: FramebufferConfig::Low,
    }) {
        Ok(video) => video,
        Err(_) => { panic!("Cannot open Video."); },
    };
    video.set_window_title("Window Test");
    loop {
        video.wait_event();  // should be waiting for refresh rate, or similar
        while let Some(event) = video.next_event() {
            match event {
                Event::KeyPress(k) => {
                    println!("KeyPress {}",k);
                },
                Event::KeyRelease(k) => {
                    println!("KeyRelease {}",k);
                },
                Event::MousePress(x,y,b) => {
                    println!("MousePress {},{}; {}",x,y,b);
                },
                Event::MouseRelease(x,y,b) => {
                    println!("MouseRelease {},{}; {}",x,y,b);
                },
                Event::MouseMove(x,y) => {
                    println!("MouseMove {},{}",x,y);
                },
                Event::MouseWheel(b) => {
                    println!("MouseWheel {}",b);
                },
                Event::Resize(width,height) => {
                    println!("Resize {}x{}",width,height);
                },
                Event::Paint(x,y,width,height) => {
                    println!("Paint {},{}; {}x{}",x,y,width,height);
                },
                Event::Close => {
                    println!("Close");
                    return;
                }
            }    
        }
    }
}