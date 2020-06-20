// G - OpenGL - Texture2D
// Desmond Germans, 2020

use gl::types::GLuint;
use crate::Image;
use std::ffi::c_void;

pub struct Texture2D {
    pub tex: GLuint,
}

impl Texture2D {
    pub fn new(width: u32,height: u32) -> Texture2D {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,gl::RGBA8,width as i32,height as i32);
        };
        Texture2D {
            tex: tex,
        }
    }
    
    pub fn upload(&mut self,x: u32,y: u32,image: &Image) {
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D,0,x as i32,y as i32,image.width as i32,image.height as i32,gl::BGRA,gl::UNSIGNED_INT_8_8_8_8_REV,image.data.as_ptr() as *const u32 as *const c_void);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.tex);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}
