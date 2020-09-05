// G - Map test
// Desmond Germans, 2020

/*use e::*;
use g::*;
use std::{
    rc::Rc,
    fs::File,
    io::prelude::*,
    time::Instant,
};*/

fn main() {
    /*// open system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create GPU graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create game engine
    let engine = Rc::new(Engine::new(&system,&graphics,vec2!(1024,576),vec2!(256,144)).expect("Cannot open engine."));

    // create map layer for game engine
    let layer = Rc::new(MapLayer::new(&engine).expect("cannot create layer"));

    // load atlas texture
    let mut file = File::open("try/8x8tiles.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let atlas_mat = image::decode::<pixel::ARGB8>(&buffer).expect("unable to decode");
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
    while engine.is_running() {
        let start_us = time.elapsed().as_micros();
        engine.update();
        let update_us = time.elapsed().as_micros();
        engine.render(&layers);
        let render_us = time.elapsed().as_micros();
        engine.present();
        let present_us = time.elapsed().as_micros();
        println!("{:016}: UPD {:006}  REN {:006}  PRS {:006}",present_us,update_us - start_us,render_us - update_us,present_us - render_us);
    }*/
}
