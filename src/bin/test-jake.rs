// G - Jake test
// Desmond Germans, 2020

use e::*;
use g::*;
use std::{
    rc::Rc,
    fs::File,
    io::prelude::*,
    time::Instant,
};

const MS_PER_US: f32 = 0.001;
const WALK_SPEED: f32 = 0.001;
const PIXELS_PER_CYCLE: f32 = 16.0;
const FRAMES_PER_CYCLE: f32 = 8.0;

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
    let mut file = File::open("try/8x8tiles.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let atlas_mat = image::decode::<pixel::ARGB8>(&buffer).expect("unable to decode");
    layer.set_atlas(&atlas_mat);

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
    layer.set_map(&map);

    // load jake texture
    let mut file = File::open("try/jake/all4.png").expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let jake_mat = image::decode::<pixel::ARGB8>(&buffer).expect("unable to decode");
    let jake_texture = Rc::new(gpu::Texture2D::<pixel::ARGB8>::new_from_mat(&graphics,&jake_mat).expect("Unable to upload Jake texture."));

    // create Jake shader
    let jake_vs = r#"
        #version 420 core
        layout(location = 0) in vec2 v_pos;
        uniform uvec2 u_cells;
        uniform uvec2 u_cell;
        uniform uvec2 u_size;
        uniform uvec2 u_pos;
        uniform uvec2 u_jake;
        out vec2 f_albedo_tex;
        out vec2 f_normal_tex;
        void main() {
            vec2 cell_size = vec2(1.0 / float(u_cells.x),1.0 / float(u_cells.y));
            vec2 albedo_cell = vec2(float(u_cell.x) * cell_size.x,float(u_cell.y) * cell_size.y);
            vec2 normal_cell = vec2(float(u_cell.x) * cell_size.x,float(u_cell.y + 4) * cell_size.y);
            f_albedo_tex = vec2(albedo_cell.x + v_pos.x * cell_size.x,albedo_cell.y + v_pos.y * cell_size.y);
            f_normal_tex = vec2(normal_cell.x + v_pos.x * cell_size.x,normal_cell.y + v_pos.y * cell_size.y);
            vec2 psize = vec2(1.0 / float(u_size.x),1.0 / float(u_size.y));
            vec2 pos = vec2(float(u_pos.x) * psize.x,float(u_pos.y) * psize.y);
            vec2 jake = vec2(float(u_jake.x) * psize.x,float(u_jake.y) * psize.y);
            gl_Position = vec4(-1.0 + 2.0 * (pos.x + v_pos.x * jake.x),-1.0 + 2.0 * (pos.y + v_pos.y * jake.y),0.0,1.0);
        }
    "#;
    let jake_fs = r#"
        #version 420 core
        uniform sampler2D u_texture;
        uniform vec3 u_ambient;
        uniform vec3 u_light;
        in vec2 f_albedo_tex;
        in vec2 f_normal_tex;
        out vec4 fs_output;
        void main() {
            vec4 albedo = texture2D(u_texture,f_albedo_tex);
            vec4 normal = vec4(-1.0,-1.0,-1.0,-1.0) + 2.0 * texture2D(u_texture,f_normal_tex);
            normal.y = -normal.y;
            float diff = 1.0;
            if(normal.w > 0.0) {
                diff = dot(normal.xyz,u_light);
            }
            vec3 result = albedo.xyz * (u_ambient + diff);
            fs_output = vec4(result,albedo.w);
        }
    "#;
    let jake_shader = match gpu::Shader::new(&graphics,jake_vs,None,jake_fs) {
        Ok(shader) => shader,
        Err(_) => { panic!("Unable to create Jake shader."); },
    };

    // create layer collection
    let layers: Vec<Rc<dyn Layer>> = vec![layer];

    // initialize jake
    let mut dir = 0;
    let mut step = 0;
    let mut sf = 0.0f32;
    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut left_pressed = false;
    let mut right_pressed = false;
    let mut posf = vec2!(50.0f32,50.0f32);
    let mut pos = vec2!(50u32,50u32);

    // main loop
    let time = Instant::now();
    let mut prev_frame_us = 1;

    while engine.is_running() {

        let frame_us = time.elapsed().as_micros();
        let delta = (frame_us - prev_frame_us) as u32;  // calculate duration of previous frame (in us)
        let df_ms = MS_PER_US * (delta as f32);  // calculate duration of previous frame (in fractional ms)

        for event in system.poll(&engine.window) {
            match event {
                Event::KeyPress(k) => {
                    match k {
                        111 => { up_pressed = true; },
                        116 => { down_pressed = true; },
                        113 => { left_pressed = true; },
                        114 => { right_pressed = true; },
                        _ => { },
                    }
                },

                Event::KeyRelease(k) => {
                    match k {
                        111 => { up_pressed = false; },
                        116 => { down_pressed = false; },
                        113 => { left_pressed = false; },
                        114 => { right_pressed = false; },
                        _ => { },
                    }
                },

                Event::Resize(s) => {
                    engine.window.size.set(vec2!(s.x as usize,s.y as usize));
                },

                Event::Close => {
                    engine.running.set(false);
                },

                _ => { },
            }
        }

        if up_pressed || down_pressed || left_pressed || right_pressed {
            if up_pressed {
                dir = 1;
            }
            else if down_pressed {
                dir = 0;
            }
            else if left_pressed {
                dir = 2;
            }
            else if right_pressed {
                dir = 3;
            }
            sf += WALK_SPEED * df_ms;  // walking animation cycle speed
            while sf >= 1.0 { sf -= 1.0; }  // or whatever other way to remove the integer part
            step = (sf * (FRAMES_PER_CYCLE - 1.0)) as u32;  // multiplier for the right frame
            let dd = WALK_SPEED * PIXELS_PER_CYCLE * df_ms;  // walking progress speed (traverse 16 pixels during one cycle)
            match dir {
                1 => { posf.y -= dd; },
                0 => { posf.y += dd; },
                2 => { posf.x -= dd; },
                3 => { posf.x += dd; },
                _ => { },
            }
            pos.x = posf.x as u32;
            pos.y = posf.y as u32;
        }

        let update_us = time.elapsed().as_micros();

        layers[0].render();
        graphics.bind_texture(0,&*jake_texture);
        graphics.bind_shader(&jake_shader);
        graphics.set_uniform("u_texture",0);
        graphics.set_uniform("u_cells",vec2!(FRAMES_PER_CYCLE as u32,8u32));  // total number of cells in the atlas
        graphics.set_uniform("u_cell",vec2!(step,dir));  // requested cell
        let size = engine.framebuffer.size;
        graphics.set_uniform("u_size",vec2!(size.x as u32,size.y as u32));  // size (in pixels) of the framebuffer
        graphics.set_uniform("u_pos",pos);  // position (in pixels) of the sprite
        graphics.set_uniform("u_jake",vec2!(16,24));  // size (in pixels) of one cell
        graphics.set_uniform("u_ambient",vec3!(0.4f32,0.4f32,0.4f32));  // ambient lighting color
        let light = vec3!(-2.0f32,-5.0f32,1.0f32).norm();  // direction of the light
        graphics.set_uniform("u_light",light);
        graphics.bind_vertexbuffer(&engine.quad_vertexbuffer);
        graphics.set_blend(gpu::BlendMode::Over);
        graphics.draw_triangle_fan(4);

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
