// G - Engine
// Desmond Germans, 2020

use crate::*;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Engine<'a> {
    pub system: Rc<System<'a>>,
    pub layers: RefCell<Vec<Layer>>,
    framebuffer: Framebuffer,
    layer_shader: Shader,
    final_shader: Shader,
    pub quad_vertexbuffer: VertexBuffer<Vec2<f32>>,
    pub running: bool,
}

static QUAD: [Vec2<f32>; 4] = [
    Vec2::<f32> { x: 0.0,y: 0.0, },
    Vec2::<f32> { x: 1.0,y: 0.0, },
    Vec2::<f32> { x: 1.0,y: 1.0, },
    Vec2::<f32> { x: 0.0,y: 1.0, },
];

pub enum EngineError {
    Generic
}

impl Debug for EngineError {
    fn fmt(&self,f: &mut Formatter) -> std::fmt::Result {
        write!(f,"EngineError::Generic")
    }
}

impl<'a> Engine<'a> {
    pub fn new(system: &Rc<System<'a>>,winsize: Vec2<usize>,fbsize: Vec2<usize>) -> Result<Engine<'a>,EngineError> {

        let framebuffer = match system.create_framebuffer(fbsize) {
            Ok(framebuffer) => framebuffer,
            Err(_) => { return Err(EngineError::Generic) },
        };

        let layer_vs = r#"
            #version 420 core
            uniform vec4 u_rect;
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
            }
        "#;
        let layer_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            in vec2 f_tex;
            out vec4 fs_output;
            void main() {
                fs_output = texture2D(u_texture,f_tex);
            }
        "#;
        let layer_shader = match system.create_shader(layer_vs,None,layer_fs) {
            Ok(shader) => shader,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let final_vs = r#"
            #version 420 core
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * v_pos.x,1.0 - 2.0 * v_pos.y,0.0,1.0);  // last stage swaps Y-output
            }
        "#;
        let final_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            in vec2 f_tex;
            layout(location = 0) out vec4 fs_output;
            void main() {
                fs_output = texture2D(u_texture,f_tex);
            }
        "#;
        let final_shader = match system.create_shader(final_vs,None,final_fs) {
            Ok(shader) => shader,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let quad_vertexbuffer = match system.create_vertexbuffer(QUAD.to_vec()) {
            Ok(vertexbuffer) => vertexbuffer,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let engine = Engine {
            system: Rc::clone(system),
            layers: RefCell::new(Vec::new()),
            framebuffer: framebuffer,
            layer_shader: layer_shader,
            final_shader: final_shader,
            quad_vertexbuffer: quad_vertexbuffer,
            running: true,
        };

        Ok(engine)
    }

    pub fn wait(&self) {
        self.system.wait();
    }

    pub fn pump(&self) {
        self.system.pump();
    }

    pub fn running(&self) -> bool {
        self.running
    }
}

impl<'a> Handler for Engine<'a> {
    fn handle(&mut self,event: Event) {
        match event {
            Event::Paint(_size,_r) => {
                self.system.bind_framebuffer(&self.framebuffer);
                for layer in &*self.layers.borrow() {
                    self.system.bind_framebuffer_as_texture2d(0,&layer.framebuffer);
                    self.system.bind_shader(&self.layer_shader);
                    self.system.set_uniform("u_texture",0);
                    self.system.bind_vertexbuffer(&self.quad_vertexbuffer);
                    self.system.draw_triangle_fan(4);
                    self.system.unbind_vertexbuffer();
                    self.system.unbind_shader();
                    self.system.unbind_texture2d(0);
                }
                self.system.unbind_framebuffer();
                self.system.bind_framebuffer_as_texture2d(0,&self.framebuffer);
                self.system.bind_shader(&self.final_shader);
                self.system.set_uniform("u_texture",0);
                self.system.bind_vertexbuffer(&self.quad_vertexbuffer);
                self.system.draw_triangle_fan(4);
                self.system.unbind_vertexbuffer();
                self.system.unbind_shader();
                self.system.unbind_texture2d(0);
            },
            Event::Close => {
                self.running = false;
            },
            _ => {
            },
        }
    }
}