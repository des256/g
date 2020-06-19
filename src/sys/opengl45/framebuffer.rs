// Kvasir - System Interface - OpenGL - Framebuffer
// Desmond Germans, 2020

extern crate gl;
use gl::types;

pub struct Framebuffer {
    fbo: types::GLuint,
    pub tex: types::GLuint,
    pub width: u32,
    pub height: u32,
}

impl Framebuffer {
    pub fn new(width: u32,height: u32) -> Framebuffer {
        let mut fbo: types::GLuint = 0;
        let mut tex: types::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1,&mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER,fbo);
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::NEAREST as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,gl::RGBA8,width as i32,height as i32);
            gl::FramebufferTexture(gl::FRAMEBUFFER,gl::COLOR_ATTACHMENT0,tex,0);
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("(linux) framebuffer incomplete");
            }
        }
        Framebuffer {
            fbo: fbo,
            tex: tex,
            width: width,
            height: height,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER,self.fbo);
            gl::Viewport(0,0,self.width as i32,self.height as i32);
            gl::Scissor(0,0,self.width as i32,self.height as i32);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER,0);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1,&self.fbo);
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
