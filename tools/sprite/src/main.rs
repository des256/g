// G Sprite Editor
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

mod document;
mod editcanvas;
mod application;

use application::*;

fn main() {
    let system = Rc::new(System::new().expect("Cannot open system."));
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open GPU."));
    let mut ui = ui::UI::new(&system,&graphics,"../../../e/static/fonts").expect("Cannot open UI.");
    let app = Rc::new(Application::new(&ui.state).expect("Cannot create main application."));
    ui.open_frame(rect!(50,50,1280,640),"Sprite Editor",&app);
    ui.run();
    ui.close(&app);
}