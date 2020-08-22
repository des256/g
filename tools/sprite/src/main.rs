// G Sprite Editor
// Desmond Germans, 2020

use e::*;
use std::rc::Rc;

mod document;
mod editcanvas;
mod application;

use application::*;

fn main() {
    // open system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create GPU graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));
    
    // create UI context
    let ui = Rc::new(ui::UI::new(&system,&graphics,"../../../e/static/fonts").expect("Cannot open UI."));

    // create main widget
    let widget = Rc::new(Application::new(&ui).expect("Cannot create main application."));

    // open host window
    ui.open(&(widget as Rc<dyn ui::Widget>),rect!(50,50,1280,640),"Sprite Editor");

    // run UI loop
    ui.run();
}