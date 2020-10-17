// G Sprite Editor
// Desmond Germans, 2020

pub use base::*;
pub mod e {
    pub use platform::*;
    pub use gpu::*;
    pub use imageformats::*;
    pub use ui::*;
}
use std::rc::Rc;

mod document;
use document::*;

mod editcanvas;
use editcanvas::*;

mod application;
use application::*;

const FONT_DIR: &str = "/home/desmond/e/static/fonts";

fn main() -> Result<(),SystemError> {
    let system = e::System::new()?;
    let graphics = e::Graphics::new(&system)?;
    let ui = e::UI::new(&system,&graphics,FONT_DIR)?;
    let app = Rc::new(Application::new(&ui)?);
    let window = e::UIWindow::new_frame(&ui,rect!(50,50,1280,640),"Sprite Editor",app as Rc<dyn e::Widget>)?;
    window.window.show();
    ui.run();
    window.window.hide();
    drop(window);
    Ok(())
}