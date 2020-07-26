// G - Engine
// Desmond Germans, 2020

use crate::*;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;

/// Game engine context.
pub struct Engine {
    pub(crate) system: Rc<System>,
    pub(crate) graphics: Rc<gpu::Graphics>,
    pub(crate) window: Rc<Window>,
    pub layers: RefCell<Vec<Rc<Layer>>>,
    pub(crate) framebuffer: Rc<gpu::Framebuffer>,
    pub(crate) layer_shader: gpu::Shader,
    pub(crate) final_shader: gpu::Shader,
    pub quad_vertexbuffer: gpu::VertexBuffer<Vec2<f32>>,
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

impl Engine {
    /// Create new game engine context.
    /// # Arguments
    /// * `system` - System to create game engine for.
    /// * `graphics` - GPU Graphics context to create game engine for.
    /// * `winsize` - Initial screen window size.
    /// * `fbsize` - Compositing framebuffer size.
    pub fn new(system: &Rc<System>,graphics: &Rc<gpu::Graphics>,winsize: Vec2<usize>,fbsize: Vec2<usize>) -> Result<Engine,EngineError> {

        let window = Rc::new(match Window::new(&system,rect!(50,50,winsize.x as isize,winsize.y as isize),"Engine Window") {
            Ok(window) => window,
            Err(_) => { return Err(EngineError::Generic); },
        });

        let framebuffer = Rc::new(match gpu::Framebuffer::new(&graphics,fbsize) {
            Ok(framebuffer) => framebuffer,
            Err(_) => { return Err(EngineError::Generic); },
        });

        let layer_vs = r#"
            #version 420 core
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
        let layer_shader = match gpu::Shader::new(&graphics,layer_vs,None,layer_fs) {
            Ok(shader) => shader,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let final_vs = r#"
            #version 420 core
            uniform vec2 u_scale;
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,v_pos.y);
                gl_Position = vec4(u_scale.x * (-1.0 + 2.0 * v_pos.x),u_scale.y * (1.0 - 2.0 * v_pos.y),0.0,1.0);  // last stage swaps Y-output
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
        let final_shader = match gpu::Shader::new(&graphics,final_vs,None,final_fs) {
            Ok(shader) => shader,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let quad_vertexbuffer = match gpu::VertexBuffer::new(&graphics,QUAD.to_vec()) {
            Ok(vertexbuffer) => vertexbuffer,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let engine = Engine {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            window: window,
            layers: RefCell::new(Vec::new()),
            framebuffer: framebuffer,
            layer_shader: layer_shader,
            final_shader: final_shader,
            quad_vertexbuffer: quad_vertexbuffer,
        };

        Ok(engine)
    }

    /// Run the engine.
    pub fn run(&self) {
        let mut running = true;
        while running {
            self.system.wait();
            for event in self.system.poll(&self.window) {
                match event {
                    Event::Render => {
                        let fb_aspect = (self.framebuffer.size.x as f32) / (self.framebuffer.size.y as f32);
                        let win_aspect = (self.window.size.get().x as f32) / (self.window.size.get().y as f32);
                        let scale = if win_aspect > fb_aspect {
                            vec2!(fb_aspect / win_aspect,1.0)
                        }
                        else {
                            vec2!(1.0,win_aspect / fb_aspect)
                        };
                        self.graphics.bind_target(&self.framebuffer);
                        for layer in &*self.layers.borrow() {
                            self.graphics.bind_texture(0,&*layer.framebuffer);
                            self.graphics.bind_shader(&self.layer_shader);
                            self.graphics.set_uniform("u_texture",0);
                            self.graphics.bind_vertexbuffer(&self.quad_vertexbuffer);
                            self.graphics.draw_triangle_fan(4);
                        }
                        self.graphics.bind_target(&self.window);
                        self.graphics.bind_texture(0,&*self.framebuffer);
                        self.graphics.bind_shader(&self.final_shader);
                        self.graphics.set_uniform("u_scale",scale);
                        self.graphics.set_uniform("u_texture",0);
                        self.graphics.bind_vertexbuffer(&self.quad_vertexbuffer);
                        self.graphics.draw_triangle_fan(4);
                        self.graphics.flush();
                    },
    
                    Event::Resize(s) => {
                        self.window.size.set(vec2!(s.x as usize,s.y as usize));
                    },
    
                    Event::Close => {
                        running = false;
                    },
    
                    _ => { },
                }
            }
        }    
    }
}
