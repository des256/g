// G - Image test
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
    let mut file = File::open("try/256x144.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let image = image::decode::<pixel::ARGB8>(&buffer).expect("unable to decode");
    let texture = gpu::Texture2D::new(&graphics,&image).expect("unable to load texture");

    // create shader for the image
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
        uniform sampler2D u_texture;
        in vec2 f_tex;
        out vec4 fs_output;
        void main() {
            fs_output = texture2D(u_texture,f_tex);
        }
    "#;
    let shader = gpu::Shader::new(&graphics,vs,None,fs).expect("cannot create shader");
    
    // draw image onto layer
    graphics.bind_target(&layer.framebuffer);
    graphics.clear(pixel::ARGB8::from(0xFFFFFF00));
    graphics.bind_texture(0,&texture);
    graphics.bind_shader(&shader);
    graphics.set_uniform("u_texture",0);
    graphics.bind_vertexbuffer(&engine.quad_vertexbuffer);
    graphics.draw_triangle_fan(4);

    // add layer to engine
    engine.layers.borrow_mut().push(Rc::clone(&layer));

    // run the engine
    engine.run();
}
