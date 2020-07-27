// G - Window test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::{
    rc::Rc,
    time::Instant,
};

fn main() {
    // open system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create GPU graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create game engine
    let engine = Rc::new(Engine::new(&system,&graphics,vec2!(1024,576),vec2!(256,144)).expect("Cannot open engine."));

    // create layer collection
    let layers: Vec<Rc<dyn Layer>> = Vec::new();

    // main loop
    let time = Instant::now();
    while engine.is_running() {
        let start_us = time.elapsed().as_micros();
        engine.update();
        let update_us = time.elapsed().as_micros();
        engine.render(&layers);
        let render_us = time.elapsed().as_micros();
        engine.present();
        let present_us = time.elapsed().as_micros();
        println!("{:016}: UPD {:006}  REN {:006}  PRS {:006}",present_us,update_us - start_us,render_us - update_us,present_us - render_us);
    }
}
