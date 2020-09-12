// G - Map test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::{
    rc::Rc,
    time::Instant,
};

const MS_PER_US: f32 = 0.001;

fn main() {
    // open system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create GPU graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create game engine
    let engine = Rc::new(Engine::new(&system,&graphics,vec2!(1024,576),vec2!(256,144)).expect("Cannot open engine."));
    
    // create map layer for game engine
    let layer = Rc::new(MapLayer::new(&engine).expect("cannot create layer"));

    // load atlas texture
    let atlas_mat = image::load::<pixel::ARGB8>("try/8x8tiles.png").expect("unable to load tiles");
    layer.set_atlas_from_mat(atlas_mat);

    // create map texture
    let mut map = Mat::<u32>::new(vec2!(4,4));
    map.set(vec2!(0,0),1);
    map.set(vec2!(1,0),1);
    map.set(vec2!(2,0),1);
    map.set(vec2!(3,0),0);
    map.set(vec2!(0,1),1);
    map.set(vec2!(1,1),3);
    map.set(vec2!(2,1),1);
    map.set(vec2!(3,1),0);
    map.set(vec2!(0,2),1);
    map.set(vec2!(1,2),1);
    map.set(vec2!(2,2),1);
    map.set(vec2!(3,2),0);
    map.set(vec2!(0,3),2);
    map.set(vec2!(1,3),2);
    map.set(vec2!(2,3),2);
    map.set(vec2!(3,3),2);
    layer.set_map_from_mat(map);

    // create layer collection
    let layers: Vec<Rc<dyn Layer>> = vec![layer];
    
    // main loop
    let time = Instant::now();
    let mut prev_frame_us = 1;
    
    while engine.is_running() {
        let frame_us = time.elapsed().as_micros();
        let delta = (frame_us - prev_frame_us) as u32;  // calculate duration of previous frame (in us)
        let df_ms = MS_PER_US * (delta as f32);  // calculate duration of previous frame (in fractional ms)

        engine.update(&layers);

        let update_us = time.elapsed().as_micros();

        engine.render(&layers);

        let render_us = time.elapsed().as_micros();

        engine.present();

        let present_us = time.elapsed().as_micros();

        println!("update: {:5}  draw: {:5}  ({:5} remaining)  {:2} Hz",
            update_us - frame_us,
            render_us - update_us,
            present_us - render_us,
            (1000.0 / df_ms) as u32,
        );
        
        prev_frame_us = frame_us;
    }
}
