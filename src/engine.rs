// G - Engine
// Desmond Germans, 2020

use crate::*;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::cell::Cell;
use std::rc::Rc;

/// Game engine context.
pub struct Engine {
    pub(crate) system: Rc<System>,
    pub(crate) graphics: Rc<gpu::Graphics>,
    pub window: Rc<Window>,
    pub framebuffer: Rc<gpu::Framebuffer>,
    pub(crate) layer_shader: gpu::Shader,
    pub(crate) final_shader: gpu::Shader,
    pub(crate) static_shader: gpu::Shader,
    pub(crate) map_shader: gpu::Shader,
    pub quad_vertexbuffer: gpu::VertexBuffer<Vec2<f32>>,
    pub running: Cell<bool>,
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

        let static_vs = r#"
            #version 420 core
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
            }
        "#;
        let static_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            in vec2 f_tex;
            out vec4 fs_output;
            void main() {
                fs_output = texture2D(u_texture,f_tex);
            }
        "#;
        let static_shader = match gpu::Shader::new(&graphics,static_vs,None,static_fs) {
            Ok(shader) => shader,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let map_vs = r#"
            #version 420 core
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
            }
        "#;
        let map_fs = r#"
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
                fs_output = d;
            }
        "#;
        let map_shader = match gpu::Shader::new(&graphics,map_vs,None,map_fs) {
            Ok(shader) => shader,
            Err(_) => { return Err(EngineError::Generic); },
        };

        let quad_vertexbuffer = match gpu::VertexBuffer::new_from_vec(&graphics,QUAD.to_vec()) {
            Ok(vertexbuffer) => vertexbuffer,
            Err(_) => { return Err(EngineError::Generic); },
        };

        Ok(Engine {
            system: Rc::clone(system),
            graphics: Rc::clone(graphics),
            window: window,
            framebuffer: framebuffer,
            layer_shader: layer_shader,
            final_shader: final_shader,
            static_shader: static_shader,
            map_shader: map_shader,
            quad_vertexbuffer: quad_vertexbuffer,
            running: Cell::new(true),
        })
    }

    pub fn is_running(&self) -> bool {
        self.running.get()
    }

    pub fn update(&self) {
        for event in self.system.poll(&self.window) {
            match event {
                Event::Resize(s) => {
                    self.window.size.set(vec2!(s.x as usize,s.y as usize));
                },

                Event::Close => {
                    self.running.set(false);
                },

                _ => { },
            }
        }
    }

    pub fn render(&self,layers: &Vec<Rc<dyn Layer>>) {
        let fb_aspect = (self.framebuffer.size.x as f32) / (self.framebuffer.size.y as f32);
        let win_aspect = (self.window.size.get().x as f32) / (self.window.size.get().y as f32);
        let scale = if win_aspect > fb_aspect {
            vec2!(fb_aspect / win_aspect,1.0)
        }
        else {
            vec2!(1.0,win_aspect / fb_aspect)
        };
        self.graphics.bind_target(&self.framebuffer);
        for layer in layers.iter() {
            self.graphics.bind_texture(0,layer.framebuffer());
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
    }

    pub fn present(&self) {
        gpu::present(&self.system,&self.window);
    }
}
