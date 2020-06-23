// G - OpenGL - Texture2D
// Desmond Germans, 2020

use gl::types::GLuint;
use crate::Image;
use std::ffi::c_void;
use std::marker::PhantomData;
use gl::types::GLenum;
use crate::RGB8;
use crate::ARGB8;

pub trait OpenGLFormat {
    fn gl_internal_format() -> GLuint;
    fn gl_format() -> GLuint;
    fn gl_type() -> GLenum;
}

impl OpenGLFormat for RGB8 {
    fn gl_internal_format() -> GLuint { gl::RGB8 as GLuint }
    fn gl_format() -> GLenum { gl::BGR }
    fn gl_type() -> GLenum { gl::UNSIGNED_BYTE }
}

impl OpenGLFormat for ARGB8 {
    fn gl_internal_format() -> GLuint { gl::RGBA8 as GLuint }
    fn gl_format() -> GLenum { gl::BGRA }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT_8_8_8_8_REV }
}

pub struct Texture2D<T: OpenGLFormat> {
    pub tex: GLuint,
    phantom: PhantomData<T>,
}

pub trait Texture2DUpload<T> {
    fn upload(&mut self,x: isize,y: isize,source: &T);
}

impl<T: OpenGLFormat> Texture2D<T> {
    pub fn new(width: usize,height: usize) -> Texture2D<T> {
        let mut tex: GLuint = 0;
        unsafe {
            gl::GenTextures(1,&mut tex);
            gl::BindTexture(gl::TEXTURE_2D,tex);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_S,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_WRAP_T,gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MIN_FILTER,gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D,gl::TEXTURE_MAG_FILTER,gl::LINEAR as i32);
            gl::TexStorage2D(gl::TEXTURE_2D,1,T::gl_internal_format(),width as i32,height as i32);
        };
        Texture2D {
            tex: tex,
            phantom: PhantomData,
        }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D,self.tex);
        }
    }
}

impl<T: OpenGLFormat> Drop for Texture2D<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1,&self.tex);
        }
    }
}

impl<T: OpenGLFormat> Texture2DUpload<Image<T>> for Texture2D<T> {
    fn upload(&mut self,x: isize,y: isize,source: &Image<T>) {
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D,0,x as i32,y as i32,source.width as i32,source.height as i32,T::gl_format(),T::gl_type(),source.data.as_ptr() as *const c_void);
        }
    }    
}
