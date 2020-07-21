// G - Image test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let system = Rc::new(match System::new() {
        Ok(system) => system,
        Err(_) => { panic!("Cannot open system."); },
    });

    let engine = match Engine::new(&system,vec2!(1024,576),vec2!(256,144)) {
        Ok(engine) => engine,
        Err(_) => { panic!("Cannot open engine."); },
    };

    let layer = Layer::new(&engine,rect!(0,0,256,144)).expect("cannot create layer");
    let mut file = File::open("try/256x144.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let image = decode::<ARGB8>(&buffer).expect("unable to decode");
    let texture = engine.system.create_texture2d::<ARGB8>(&image).expect("unable to load texture");

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
    let shader = engine.system.create_shader(vs,None,fs).expect("cannot create shader");

    engine.bind_layer(&layer);
    engine.system.clear(RGB8::from(0xFFFFFF00));
    engine.system.bind_texture2d(0,&texture);
    engine.system.bind_shader(&shader);
    engine.system.set_uniform("u_texture",0);
    engine.system.bind_vertexbuffer(&engine.quad_vertexbuffer);
    engine.system.draw_triangle_fan(4);
    engine.unbind_layer();

    engine.layers.borrow_mut().push(layer);

    while engine.running {
        system.wait();
        system.pump();
    }
}
