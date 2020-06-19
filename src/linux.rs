// Kvasir - Linux
// Desmond Germans, 2020

use std::ffi;
use std::ptr;
use std::os::raw;

extern crate x11;

use x11::xlib;
use x11::glx;
use gl::types;

use crate::*;

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(
    dpy: *mut xlib::Display,
    fbc: glx::GLXFBConfig,
    share_context: glx::GLXContext,
    direct: xlib::Bool,
    attribs: *const raw::c_int
) -> glx::GLXContext;

fn load_function(name: &str) -> *mut raw::c_void {
    let newname = ffi::CString::new(name).unwrap();
    let pointer: *mut raw::c_void = unsafe {
        std::mem::transmute(
            glx::glXGetProcAddress(newname.as_ptr() as *const u8)
        )
    };
    if pointer.is_null() {
        panic!("(linux, ui+opengl) unable to access {}",name);
    }
    pointer
}

pub enum WindowConfig {
    Standard,  // 640x360
    High,  // 1280x720
}

pub enum FramebufferConfig {
    Standard,  // 640x360
    Low,  // 320x180
}

pub struct VideoConfig {
    pub window: WindowConfig,
    pub framebuffer: FramebufferConfig,
}

pub struct Video {
    pub layers: Vec<Layer>,
    framebuffer: Framebuffer,
    connection: xcb::Connection,
    window: u32,
    window_width: u32,
    window_height: u32,
    context: glx::GLXContext,
    wm_delete_window: u32,
    layer_shader: Shader,
    final_shader: Shader,
    pub quad_vbo: types::GLuint,
}

pub enum Button {
    Left,
    Middle,
    Right,
}

impl std::fmt::Display for Button {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Button::Left => { write!(f,"left") },
            Button::Middle => { write!(f,"middle") },
            Button::Right => { write!(f,"right") },
        }
    }
}

pub enum Wheel {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Wheel {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Wheel::Up => { write!(f,"up") },
            Wheel::Down => { write!(f,"down") },
            Wheel::Left => { write!(f,"left") },
            Wheel::Right => { write!(f,"right") },
        }
    }
}

pub enum Event {
    KeyPress(u8),
    KeyRelease(u8),
    MousePress(i32,i32,Button),
    MouseRelease(i32,i32,Button),
    MouseWheel(Wheel),
    MouseMove(i32,i32),
    Paint(i32,i32,i32,i32),
    Geometry(i32,i32,i32,i32),
    Close,
}

static QUAD: [types::GLfloat; 8] = [
    0.0,0.0,
    1.0,0.0,
    1.0,1.0,
    0.0,1.0,
];

pub enum VideoError {
    X11Error,
    GLXError,
    GLXVersionMismatch,
    NoSuitableConfig,
    MissingGLXARBCreateContext,
    MissingWMProtocols,
    MissingWMDeleteWindow,
    OpenGLError,
    OpenGLIndirect,
}

impl Video {
    pub fn new(config: VideoConfig) -> std::result::Result<Video,VideoError> {

        let connection = match xcb::Connection::connect_with_xlib_display() {
            Ok((connection,_)) => connection,
            Err(_) => { return Err(VideoError::X11Error); },
        };
        connection.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

        let (visual_screen,visualid,depth,fbconfig,glx_create_context_attribs) = {
            let mut glxmaj: raw::c_int = 0;
            let mut glxmin: raw::c_int = 0;
            unsafe {
                if glx::glXQueryVersion(
                    connection.get_raw_dpy(),
                    &mut glxmaj as *mut raw::c_int,
                    &mut glxmin as *mut raw::c_int
                ) == 0 {
                    return Err(VideoError::GLXError);
                }
            }
            if (glxmaj * 100 + glxmin) < 103 {
                return Err(VideoError::GLXVersionMismatch);
            }
            let attribs = [
                glx::GLX_X_RENDERABLE,  1,
                glx::GLX_DRAWABLE_TYPE, glx::GLX_WINDOW_BIT,
                glx::GLX_RENDER_TYPE,   glx::GLX_RGBA_BIT,
                glx::GLX_X_VISUAL_TYPE, glx::GLX_TRUE_COLOR,
                glx::GLX_RED_SIZE,      8,
                glx::GLX_GREEN_SIZE,    8,
                glx::GLX_BLUE_SIZE,     8,
                glx::GLX_ALPHA_SIZE,    8,
                glx::GLX_DEPTH_SIZE,    24,
                glx::GLX_STENCIL_SIZE,  8,
                glx::GLX_DOUBLEBUFFER,  1,
                0,
            ];
            let mut fbcount: raw::c_int = 0;
            let fbconfigs = unsafe {
                glx::glXChooseFBConfig(
                    connection.get_raw_dpy(),
                    0,
                    attribs.as_ptr(),
                    &mut fbcount as *mut raw::c_int
                )
            };
            if fbcount == 0 {
                return Err(VideoError::NoSuitableConfig);
            }
            let fbconfig = unsafe { *fbconfigs };
            unsafe { xlib::XFree(fbconfigs as *mut raw::c_void); }
            let visual = unsafe { glx::glXGetVisualFromFBConfig(connection.get_raw_dpy(),fbconfig) };
            let screen = unsafe { (*visual).screen };
            let visual_screen = connection.get_setup().roots().nth(screen as usize).unwrap();
            let depth = unsafe { (*visual).depth };
            let visualid = unsafe { (*visual).visualid };
            let extensions = unsafe {
                ffi::CStr::from_ptr(glx::glXQueryExtensionsString(connection.get_raw_dpy(),screen))
            }.to_str().unwrap();
            let mut found = false;
            for extension in extensions.split(" ") {
                if extension == "GLX_ARB_create_context" {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(VideoError::MissingGLXARBCreateContext);
            }
            let glx_create_context_attribs: GlXCreateContextAttribsARBProc = unsafe {
                std::mem::transmute(load_function("glXCreateContextAttribsARB"))
            };
            (visual_screen,visualid,depth,fbconfig,glx_create_context_attribs)
        };

        let protocols_com = xcb::intern_atom(&connection,false,"WM_PROTOCOLS");
        let delete_window_com = xcb::intern_atom(&connection,false,"WM_DELETE_WINDOW");
        let wm_protocols = match protocols_com.get_reply() {
            Ok(protocols) => protocols.atom(),
            Err(_) => { return Err(VideoError::MissingWMProtocols); },
        };
        let wm_delete_window = match delete_window_com.get_reply() {
            Ok(delete_window) => delete_window.atom(),
            Err(_) => { return Err(VideoError::MissingWMDeleteWindow); },
        };
        
        let rootwindow = visual_screen.root();
        let window = connection.generate_id();
        let colormap = connection.generate_id();
        xcb::create_colormap(
            &connection,
            xcb::COLORMAP_ALLOC_NONE as u8,
            colormap,
            rootwindow,
            visualid as u32
        );
        let values = [
            (xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE
                | xcb::EVENT_MASK_KEY_PRESS
                | xcb::EVENT_MASK_KEY_RELEASE
                | xcb::EVENT_MASK_BUTTON_PRESS
                | xcb::EVENT_MASK_BUTTON_RELEASE
                | xcb::EVENT_MASK_POINTER_MOTION
                | xcb::EVENT_MASK_STRUCTURE_NOTIFY
            ),
            (xcb::CW_COLORMAP,colormap),
        ];
        let (window_width,window_height) = match config.window {
            WindowConfig::Standard => {
                (640,360)
            },
            WindowConfig::High => {
                (1280,720)
            }
        };
        xcb::create_window(
            &connection,
            depth as u8,
            window,
            rootwindow,
            0,0,window_width,window_height,
            0,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            visualid as u32,
            &values
        );
        unsafe {
            xcb::map_window(&connection,window);
            connection.flush();
            xlib::XSync(connection.get_raw_dpy(),xlib::False);
        }

        let protocol_set = [wm_delete_window];
        xcb::change_property(&connection,xcb::PROP_MODE_REPLACE as u8,window,wm_protocols,xcb::ATOM_ATOM,32,&protocol_set);

        let context = {
            let context_attribs: [raw::c_int; 5] = [
                glx::arb::GLX_CONTEXT_MAJOR_VERSION_ARB as raw::c_int,4,
                glx::arb::GLX_CONTEXT_MINOR_VERSION_ARB as raw::c_int,5,
                0,
            ];
            let context = unsafe {
                glx_create_context_attribs(
                    connection.get_raw_dpy(),
                    fbconfig,
                    std::ptr::null_mut(),
                    xlib::True,
                    &context_attribs[0] as *const raw::c_int
                )
            };
            connection.flush();
            unsafe { xlib::XSync(connection.get_raw_dpy(),xlib::False) };
            if context.is_null() {
                return Err(VideoError::OpenGLError);
            }
            if unsafe { glx::glXIsDirect(connection.get_raw_dpy(),context) } == 0 {
                return Err(VideoError::OpenGLIndirect);
            }
            unsafe { glx::glXMakeCurrent(connection.get_raw_dpy(),window as xlib::XID,context) };
            gl::load_with(|symbol| load_function(&symbol));
            context
        };

        let (framebuffer_width,framebuffer_height) = match config.framebuffer {
            FramebufferConfig::Standard => { (640,360) },
            FramebufferConfig::Low => { (320,180) },
        };
        let framebuffer = Framebuffer::new(framebuffer_width,framebuffer_height);

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
        let layer_shader = Shader::new(layer_vs,None,layer_fs);

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
        let final_shader = Shader::new(final_vs,None,final_fs);

        let mut quad_vbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1,&mut quad_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,quad_vbo);
            gl::BufferData(gl::ARRAY_BUFFER,32,std::mem::transmute(&QUAD[0]),gl::STATIC_DRAW);
        }

        Ok(Video {
            layers: Vec::new(),
            framebuffer: framebuffer,
            connection: connection,
            window: window,
            window_width: window_width as u32,
            window_height: window_height as u32,
            context: context,
            wm_delete_window: wm_delete_window,
            layer_shader: layer_shader,
            final_shader: final_shader,
            quad_vbo: quad_vbo,
        })
    }

    pub fn set_window_title(&self,title: &str) {
        let cs = std::ffi::CString::new(title).unwrap();
        xcb::change_property(&self.connection,xcb::PROP_MODE_REPLACE as u8,self.window,xcb::ATOM_WM_NAME,xcb::ATOM_STRING,8,cs.as_bytes());
        self.connection.flush();
    }

    pub fn next_event(&mut self) -> Option<Event> {
        while let Some(xcb_event) = self.connection.poll_for_event() {
            let r = xcb_event.response_type() & !0x80;
            match r {
                xcb::EXPOSE => {
                    unsafe {
                        glx::glXMakeCurrent(self.connection.get_raw_dpy(),self.window as u64,self.context);
                        self.framebuffer.bind();
                        for layer in &self.layers {
                            gl::Viewport(layer.x,layer.y,layer.width as i32,layer.height as i32);
                            gl::Scissor(layer.x,layer.y,layer.width as i32,layer.height as i32);
                            // TODO: begin blending
                            gl::BindTexture(gl::TEXTURE_2D,layer.framebuffer.tex);
                            self.layer_shader.bind();
                            self.layer_shader.set_uniform("u_texture",0);
                            gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
                            gl::EnableVertexAttribArray(0);
                            gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const types::GLvoid);
                            gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
                            gl::DisableVertexAttribArray(0);
                            gl::Flush();
                            // TODO: end blending
                        }
                        self.framebuffer.unbind();
                        let scale = if (self.window_width as f32) / (self.window_height as f32) > (self.framebuffer.width as f32) / (self.framebuffer.height as f32) {
                            (self.window_height as f32) / (self.framebuffer.height as f32)
                        }
                        else {
                            (self.window_width as f32) / (self.framebuffer.width as f32)
                        };
                        let width = ((self.framebuffer.width as f32) * scale) as u32;
                        let height = ((self.framebuffer.height as f32) * scale) as u32;
                        let x = (self.window_width - width) / 2;
                        let y = (self.window_height - height) / 2;
                        gl::Viewport(x as i32,y as i32,width as i32,height as i32);
                        gl::Scissor(x as i32,y as i32,width as i32,height as i32);
                        gl::BindTexture(gl::TEXTURE_2D,self.framebuffer.tex);
                        self.final_shader.bind();
                        self.final_shader.set_uniform("u_texture",0);
                        gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
                        gl::EnableVertexAttribArray(0);
                        gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const types::GLvoid);
                        gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
                        gl::DisableVertexAttribArray(0);
                        gl::Flush();
                        glx::glXSwapBuffers(self.connection.get_raw_dpy(),self.window as xlib::XID);
                    }
                },
                xcb::KEY_PRESS => {
                    let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let k = key_press.detail() as u8;
                    return Some(Event::KeyPress(k));
                },
                xcb::KEY_RELEASE => {
                    let key_release: &xcb::KeyReleaseEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let k = key_release.detail() as u8;
                    return Some(Event::KeyRelease(k));
                },
                xcb::BUTTON_PRESS => {
                    let button_press: &xcb::ButtonPressEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let x = button_press.event_x() as i32;
                    let y = button_press.event_y() as i32;
                    match button_press.detail() {
                        1 => { return Some(Event::MousePress(x,y,Button::Left)); },
                        2 => { return Some(Event::MousePress(x,y,Button::Middle)); },
                        3 => { return Some(Event::MousePress(x,y,Button::Right)); },
                        4 => { return Some(Event::MouseWheel(Wheel::Up)); },
                        5 => { return Some(Event::MouseWheel(Wheel::Down)); },
                        6 => { return Some(Event::MouseWheel(Wheel::Left)); },
                        7 => { return Some(Event::MouseWheel(Wheel::Right)); },
                        _ => { },
                    }
                },
                xcb::BUTTON_RELEASE => {
                    let button_release: &xcb::ButtonReleaseEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let x = button_release.event_x() as i32;
                    let y = button_release.event_y() as i32;
                    match button_release.detail() {
                        1 => { return Some(Event::MouseRelease(x,y,Button::Left)); },
                        2 => { return Some(Event::MouseRelease(x,y,Button::Middle)); },
                        3 => { return Some(Event::MouseRelease(x,y,Button::Right)); },
                        _ => { },
                    }
                },
                xcb::MOTION_NOTIFY => {
                    let motion_notify: &xcb::MotionNotifyEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let x = motion_notify.event_x() as i32;
                    let y = motion_notify.event_y() as i32;
                    return Some(Event::MouseMove(x,y));
                },
                xcb::CONFIGURE_NOTIFY => {
                    let configure_notify: &xcb::ConfigureNotifyEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let x = configure_notify.x() as i32;
                    let y = configure_notify.y() as i32;
                    let width = configure_notify.width() as i32;
                    let height = configure_notify.height() as i32;
                    self.window_width = width as u32;
                    self.window_height = height as u32;
                    return Some(Event::Geometry(x,y,width,height));
                },
                xcb::CLIENT_MESSAGE => {
                    let client_message : &xcb::ClientMessageEvent = unsafe { xcb::cast_event(&xcb_event) };
                    let data = &client_message.data().data;
                    let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);
                    if atom == self.wm_delete_window {
                        return Some(Event::Close);
                    }
                },
                _ => { },
            }
        }    
        None
    }
}

impl Drop for Video {
    fn drop(&mut self) {
        unsafe { glx::glXMakeCurrent(self.connection.get_raw_dpy(),0,ptr::null_mut()); }
        xcb::unmap_window(&self.connection,self.window);
        xcb::destroy_window(&self.connection,self.window);
        unsafe { glx::glXDestroyContext(self.connection.get_raw_dpy(),self.context); }
    }
}
