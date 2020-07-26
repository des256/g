// G - Map test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // open system
    let system = Rc::new(System::new().expect("Cannot open system."));

    // create GPU graphics context
    let graphics = Rc::new(gpu::Graphics::new(&system).expect("Cannot open graphics."));

    // create game engine
    let engine = Rc::new(Engine::new(&system,&graphics,vec2!(1024,576),vec2!(256,144)).expect("Cannot open engine."));

    // create layer for game engine
    let layer = Rc::new(Layer::new(&engine,rect!(0,0,256,144)).expect("cannot create layer"));

    // load image into texture
    let mut file = File::open("try/8x8tiles.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let image = image::decode::<pixel::ARGB8>(&buffer).expect("unable to decode");
    let atlas_texture = gpu::Texture2D::new(&graphics,&image).expect("unable to upload tiles texture");

    // create map texture
    let mut map = Mat::<u32>::new(vec2!(4,4));

    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),0);

    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),3);
    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),0);

    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),1);
    map.set(vec2!(0,0),0);

    map.set(vec2!(0,0),2);
    map.set(vec2!(0,0),2);
    map.set(vec2!(0,0),2);
    map.set(vec2!(0,0),2);

    let map_texture = gpu::Texture2D::new(&graphics,&map).expect("unable to upload map texture");

    // create shader
    let vs = r#"
        #version 420 core
        layout(location = 0) in vec2 v_pos;
        out vec2 f_tex;
        void main() {
            f_tex = vec2(v_pos.x,v_pos.y);
            gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
        }
    "#;
    let fs = r#"
        #version 420 core
        uniform usampler2D map_texture;
        uniform sampler2D atlas_texture;
        uniform vec2 offset;
        uniform vec2 tiles_per_pixel;
        uniform vec2 pixels_per_layer;
        uniform vec2 maps_per_tile;
        const uint TILES_PER_ATLAS = 32;
        in vec2 f_tex;
        out vec4 fs_output;
        void main() {
            vec2 tc = f_tex * pixels_per_layer * tiles_per_pixel + offset;
            vec2 mc = floor(tc) * maps_per_tile;
            uint tile_index = texture(map_texture,mc).x;
            vec2 tsc = vec2(
                float(tile_index % TILES_PER_ATLAS),
                float(tile_index / TILES_PER_ATLAS)
            );
            vec2 ftsc = tsc + fract(tc);
            vec2 ntsc = vec2(
                ftsc.x / TILES_PER_ATLAS,
                ftsc.y / TILES_PER_ATLAS
            );
            vec4 d = texture(atlas_texture,ntsc);
            fs_output = vec4(d.x,d.y,d.z,1.0);
        }
    "#;
    let shader = gpu::Shader::new(&graphics,vs,None,fs).expect("cannot create shader");
    
    // draw image onto layer
    graphics.bind_target(&layer.framebuffer);
    graphics.clear(pixel::ARGB8::from(0xFFFFFF00));
    graphics.bind_texture(0,&atlas_texture);
    graphics.bind_texture(1,&map_texture);
    graphics.bind_shader(&shader);
    graphics.set_uniform("atlas_texture",0);
    graphics.set_uniform("map_texture",1);
    graphics.set_uniform("offset",vec2!(0.125,0.125));
    graphics.set_uniform("tiles_per_pixel",vec2!(0.125,0.125));
    graphics.set_uniform("pixels_per_layer",vec2!(engine.framebuffer.size.x as f32,engine.framebuffer.size.y as f32));
    graphics.set_uniform("maps_per_tile",vec2!(0.25,0.25));
    graphics.bind_vertexbuffer(&engine.quad_vertexbuffer);
    graphics.draw_triangle_fan(4);

    // add layer to engine
    engine.layers.borrow_mut().push(Rc::clone(&layer));

    // run the engine
    engine.run();
}
