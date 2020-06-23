// G - OpenGL - Context
// Desmond Germans, 2020

use crate::FramebufferConfig;
use gl::types::GLuint;
use crate::VideoError;
use std::mem::transmute;
use gl::types::GLfloat;
use crate::Layer;
use crate::Framebuffer;
use crate::Shader;
use crate::SetUniform;
use gl::types::GLvoid;

pub struct OpenGLContext {
    pub layers: Vec<Layer>,
    framebuffer: Framebuffer,
    layer_shader: Shader,
    final_shader: Shader,
    pub quad_vbo: GLuint,
}

static QUAD: [GLfloat; 8] = [
    0.0,0.0,
    1.0,0.0,
    1.0,1.0,
    0.0,1.0,
];

impl OpenGLContext {
    pub fn new(config: FramebufferConfig) -> Result<OpenGLContext,VideoError> {
        let framebuffer = match Framebuffer::new(config.width,config.height) {
            Some(framebuffer) => framebuffer,
            None => { return Err(VideoError::Generic) },
        };
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
        }
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
        let layer_shader = match Shader::new(layer_vs,None,layer_fs) {
            Some(shader) => shader,
            None => { return Err(VideoError::Generic); },
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
        let final_shader = match Shader::new(final_vs,None,final_fs) {
            Some(shader) => shader,
            None => { return Err(VideoError::Generic); },
        };

        let mut quad_vbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1,&mut quad_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,quad_vbo);
            gl::BufferData(gl::ARRAY_BUFFER,32,transmute(&QUAD[0]),gl::STATIC_DRAW);
        }

        Ok(OpenGLContext {
            layers: Vec::new(),
            framebuffer: framebuffer,
            layer_shader: layer_shader,
            final_shader: final_shader,
            quad_vbo: quad_vbo,
        })
    }

    pub fn render(&self,window_width: u32,window_height: u32) {
        self.framebuffer.bind();
        for layer in &self.layers {
            unsafe {
                gl::Viewport(layer.x as i32,layer.y as i32,layer.width as i32,layer.height as i32);
                gl::Scissor(layer.x as i32,layer.y as i32,layer.width as i32,layer.height as i32);
                // TODO: begin blending
                gl::BindTexture(gl::TEXTURE_2D,layer.framebuffer.tex);
            }
            self.layer_shader.bind();
            self.layer_shader.set_uniform("u_texture",0);
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid);
                gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
                gl::DisableVertexAttribArray(0);
                gl::Flush();
                // TODO: end blending
            }
        }
        self.framebuffer.unbind();
        let scale = if (window_width as f32) / (window_height as f32) > (self.framebuffer.width as f32) / (self.framebuffer.height as f32) {
            (window_height as f32) / (self.framebuffer.height as f32)
        }
        else {
            (window_width as f32) / (self.framebuffer.width as f32)
        };
        let width = ((self.framebuffer.width as f32) * scale) as u32;
        let height = ((self.framebuffer.height as f32) * scale) as u32;
        let x = (window_width - width) / 2;
        let y = (window_height - height) / 2;
        unsafe {
            gl::Viewport(x as i32,y as i32,width as i32,height as i32);
            gl::Scissor(x as i32,y as i32,width as i32,height as i32);
            gl::BindTexture(gl::TEXTURE_2D,self.framebuffer.tex);
        }
        self.final_shader.bind();
        self.final_shader.set_uniform("u_texture",0);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid);
            gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
            gl::DisableVertexAttribArray(0);
            gl::Flush();
        }
    }
}
