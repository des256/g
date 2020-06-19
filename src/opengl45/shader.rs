// Kvasir - System Interface - OpenGL - Shader
// Desmond Germans, 2020

use std::{ffi,fmt,ptr};
#[doc(no_inline)]
extern crate gl;
use gl::types;

/// Shader program.
pub struct Shader {
    #[doc(hidden)]
    sp: types::GLuint,
}

pub trait SetUniform<T> {
    fn set_uniform(&self,name: &str,value: T);
}

impl Shader {
    pub fn new(
        vertex_src: &str,
        geometry_src: Option<&str>,
        fragment_src: &str,
    ) -> Shader {
        unsafe {
            // compile vertex shader
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let vcstr = ffi::CString::new(vertex_src.as_bytes()).unwrap();
            gl::ShaderSource(vs,1,&vcstr.as_ptr(),ptr::null());
            gl::CompileShader(vs);
            let mut success = gl::FALSE as types::GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);
            gl::GetShaderiv(vs,gl::COMPILE_STATUS,&mut success);
            gl::GetShaderInfoLog(vs,512,ptr::null_mut(),info_log.as_mut_ptr() as *mut types::GLchar);
            let c_str: &ffi::CStr = ffi::CStr::from_ptr(info_log.as_ptr());
            let str_slice: &str = c_str.to_str().unwrap();
            if str_slice.len() > 0 {
                println!("Shader: vertex shader errors:\n{}\nvertex shader source:\n{}",str_slice,vertex_src);
            }
            if success != gl::TRUE as types::GLint {
                panic!("Shader: aborting.");
            }

            // compile geometry shader
            let mut gs: u32 = 0;
            if let Some(geometry_src) = geometry_src {
                gs = gl::CreateShader(gl::GEOMETRY_SHADER);
                let gcstr = ffi::CString::new(geometry_src.as_bytes()).unwrap();
                gl::ShaderSource(gs,1,&gcstr.as_ptr(),ptr::null());
                gl::CompileShader(gs);
                let mut success = gl::FALSE as types::GLint;
                let mut info_log = Vec::with_capacity(512);
                info_log.set_len(512 - 1);
                gl::GetShaderiv(gs,gl::COMPILE_STATUS,&mut success);
                gl::GetShaderInfoLog(gs,512,ptr::null_mut(),info_log.as_mut_ptr() as *mut types::GLchar);
                let c_str: &ffi::CStr = ffi::CStr::from_ptr(info_log.as_ptr());
                let str_slice: &str = c_str.to_str().unwrap();
                if str_slice.len() > 0 {
                    println!("Shader: geometry shader errors:\n{}\ngeometry shader source:\n{}",str_slice,geometry_src);
                }
                if success != gl::TRUE as types::GLint {
                    panic!("Shader: aborting.");
                }
            }

            // compile fragment shader
            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fcstr = ffi::CString::new(fragment_src.as_bytes()).unwrap();
            gl::ShaderSource(fs,1,&fcstr.as_ptr(),ptr::null());
            gl::CompileShader(fs);
            gl::GetShaderiv(fs,gl::COMPILE_STATUS,&mut success);
            gl::GetShaderInfoLog(fs,512,ptr::null_mut(),info_log.as_mut_ptr() as *mut types::GLchar);
            let c_str: &ffi::CStr = ffi::CStr::from_ptr(info_log.as_ptr());
            let str_slice: &str = c_str.to_str().unwrap();
            if str_slice.len() > 0 {
                println!("Shader: fragment shader errors:\n{}\nfragment shader source:\n{}",str_slice,fragment_src);
            }
            if success != gl::TRUE as types::GLint {
                panic!("Shader: aborting.");
            }

            // link shaders
            let sp = gl::CreateProgram();
            gl::AttachShader(sp,vs);
            if gs != 0 {
                gl::AttachShader(sp,gs);
            }
            gl::AttachShader(sp,fs);
            gl::LinkProgram(sp);
            gl::GetProgramiv(sp,gl::LINK_STATUS,&mut success);
            gl::GetProgramInfoLog(sp,512,ptr::null_mut(),info_log.as_mut_ptr() as *mut types::GLchar);
            let c_str: &ffi::CStr = ffi::CStr::from_ptr(info_log.as_ptr());
            let str_slice: &str = c_str.to_str().unwrap();
            if str_slice.len() > 0 {
                println!("Shader: shader program errors:\n{}", str_slice);
            }
            if success != gl::TRUE as types::GLint {
                panic!("Shader: aborting.");
            }

            // and delete references to the separate shaders
            gl::DeleteShader(vs);
            if gs != 0 {
                gl::DeleteShader(gs);
            }
            gl::DeleteShader(fs);

            Shader {
                sp: sp,
            }
        }
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.sp); }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.sp); }
    }
}

impl fmt::Display for Shader {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(sp {})",self.sp)
    }
}

impl SetUniform<[f32; 2]> for Shader {
    fn set_uniform(&self,name: &str,value: [f32; 2]) {
        let cname = std::ffi::CString::new(name).unwrap();
        let res = unsafe { gl::GetUniformLocation(self.sp,cname.as_ptr() as *const types::GLchar) };
        unsafe {
            gl::Uniform4fv(res,1,&value as *const [f32; 2] as *const types::GLfloat);
        }
    }
}

impl SetUniform<[f32; 4]> for Shader {
    fn set_uniform(&self,name: &str,value: [f32; 4]) {
        let cname = std::ffi::CString::new(name).unwrap();
        let res = unsafe { gl::GetUniformLocation(self.sp,cname.as_ptr() as *const types::GLchar) };
        unsafe {
            gl::Uniform4fv(res,1,&value as *const [f32; 4] as *const types::GLfloat);
        }
    }
}

impl SetUniform<i32> for Shader {
    fn set_uniform(&self,name: &str,value: i32) {
        let cname = std::ffi::CString::new(name).unwrap();
        let res = unsafe { gl::GetUniformLocation(self.sp,cname.as_ptr() as *const types::GLchar) };
        unsafe {
            gl::Uniform1i(res,value);
        }
    }
}
