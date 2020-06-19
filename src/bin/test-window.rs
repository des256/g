// Kvasir - System Interface - Window test
// Desmond Germans, 2020

fn main() {
    let mut video = match kvasir::Video::new(kvasir::VideoConfig {
        window: kvasir::WindowConfig::High,
        framebuffer: kvasir::FramebufferConfig::Low,
    }) {
        Ok(video) => video,
        Err(_) => { panic!("Cannot open Video."); },
    };
    video.set_window_title("Window Test");
    loop {
        while let Some(event) = video.next_event() {
            match event {
                kvasir::Event::KeyPress(k) => {
                    println!("KeyPress {}",k);
                },
                kvasir::Event::KeyRelease(k) => {
                    println!("KeyRelease {}",k);
                },
                kvasir::Event::MousePress(x,y,b) => {
                    println!("MousePress {},{}; {}",x,y,b);
                },
                kvasir::Event::MouseRelease(x,y,b) => {
                    println!("MouseRelease {},{}; {}",x,y,b);
                },
                kvasir::Event::MouseMove(x,y) => {
                    println!("MouseMove {},{}",x,y);
                },
                kvasir::Event::MouseWheel(b) => {
                    println!("MouseWheel {}",b);
                },
                kvasir::Event::Geometry(x,y,width,height) => {
                    println!("Geometry {},{}; {}x{}",x,y,width,height);
                },
                kvasir::Event::Paint(x,y,width,height) => {
                    println!("Paint {},{}; {}x{}",x,y,width,height);
                },
                kvasir::Event::Close => {
                    return;
                }
            }    
        }
    }
}
