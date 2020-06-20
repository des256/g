// G - Linux
// Desmond Germans, 2020

use x11::xlib::*;
use std::os::raw::c_void;
use std::os::raw::c_int;
use std::ffi::CString;
use std::mem::transmute;
use crate::Layer;
use crate::Framebuffer;
use crate::VideoConfig;
use crate::WindowConfig;
use crate::FramebufferConfig;
use xcb::base::Connection;
use crate::Shader;
use gl::types::GLuint;
use gl::types::GLfloat;
use xcb::base::EventQueueOwner;
use x11::glx::*;
use std::ffi::CStr;
use xcb::xproto::*;
use x11::glx::arb::*;
use std::ptr::null_mut;
use crate::Event;
use gl::types::GLvoid;
use crate::SetUniform;
use xcb::base::cast_event;
use crate::Button;
use crate::Wheel;
use xcb::base::GenericEvent;

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(
    dpy: *mut Display,
    fbc: GLXFBConfig,
    share_context: GLXContext,
    direct: Bool,
    attribs: *const c_int
) -> GLXContext;

fn load_function(name: &str) -> *mut c_void {
    let newname = CString::new(name).unwrap();
    let pointer: *mut c_void = unsafe {
        transmute(
            glXGetProcAddress(newname.as_ptr() as *const u8)
        )
    };
    if pointer.is_null() {
        panic!("(linux, ui+opengl) unable to access {}",name);
    }
    pointer
}

pub struct Video {
    pub layers: Vec<Layer>,
    framebuffer: Framebuffer,
    connection: Connection,
    window: u32,
    window_width: u32,
    window_height: u32,
    context: GLXContext,
    wm_delete_window: u32,
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

pub enum VideoError {
    Generic,
}

impl Video {
    pub fn new(config: VideoConfig) -> Result<Video,VideoError> {

        let connection = match Connection::connect_with_xlib_display() {
            Ok((connection,_)) => connection,
            Err(_) => { return Err(VideoError::Generic); },
        };
        connection.set_event_queue_owner(EventQueueOwner::Xcb);

        let (visual_screen,visualid,depth,fbconfig,glx_create_context_attribs) = {
            let mut glxmaj: c_int = 0;
            let mut glxmin: c_int = 0;
            unsafe {
                if glXQueryVersion(
                    connection.get_raw_dpy(),
                    &mut glxmaj as *mut c_int,
                    &mut glxmin as *mut c_int
                ) == 0 {
                    return Err(VideoError::Generic);
                }
            }
            if (glxmaj * 100 + glxmin) < 103 {
                return Err(VideoError::Generic);
            }
            let attribs = [
                GLX_X_RENDERABLE,  1,
                GLX_DRAWABLE_TYPE, GLX_WINDOW_BIT,
                GLX_RENDER_TYPE,   GLX_RGBA_BIT,
                GLX_X_VISUAL_TYPE, GLX_TRUE_COLOR,
                GLX_RED_SIZE,      8,
                GLX_GREEN_SIZE,    8,
                GLX_BLUE_SIZE,     8,
                GLX_ALPHA_SIZE,    8,
                GLX_DEPTH_SIZE,    24,
                GLX_STENCIL_SIZE,  8,
                GLX_DOUBLEBUFFER,  1,
                0,
            ];
            let mut fbcount: c_int = 0;
            let fbconfigs = unsafe {
                glXChooseFBConfig(
                    connection.get_raw_dpy(),
                    0,
                    attribs.as_ptr(),
                    &mut fbcount as *mut c_int
                )
            };
            if fbcount == 0 {
                return Err(VideoError::Generic);
            }
            let fbconfig = unsafe { *fbconfigs };
            unsafe { XFree(fbconfigs as *mut c_void); }
            let visual = unsafe { glXGetVisualFromFBConfig(connection.get_raw_dpy(),fbconfig) };
            let screen = unsafe { (*visual).screen };
            let visual_screen = connection.get_setup().roots().nth(screen as usize).unwrap();
            let depth = unsafe { (*visual).depth };
            let visualid = unsafe { (*visual).visualid };
            let extensions = unsafe {
                CStr::from_ptr(glXQueryExtensionsString(connection.get_raw_dpy(),screen))
            }.to_str().unwrap();
            let mut found = false;
            for extension in extensions.split(" ") {
                if extension == "GLX_ARB_create_context" {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(VideoError::Generic);
            }
            let glx_create_context_attribs: GlXCreateContextAttribsARBProc = unsafe {
                transmute(load_function("glXCreateContextAttribsARB"))
            };
            (visual_screen,visualid,depth,fbconfig,glx_create_context_attribs)
        };

        let protocols_com = intern_atom(&connection,false,"WM_PROTOCOLS");
        let delete_window_com = intern_atom(&connection,false,"WM_DELETE_WINDOW");
        let wm_protocols = match protocols_com.get_reply() {
            Ok(protocols) => protocols.atom(),
            Err(_) => { return Err(VideoError::Generic); },
        };
        let wm_delete_window = match delete_window_com.get_reply() {
            Ok(delete_window) => delete_window.atom(),
            Err(_) => { return Err(VideoError::Generic); },
        };
        
        let rootwindow = visual_screen.root();
        let window = connection.generate_id();
        let colormap = connection.generate_id();
        create_colormap(
            &connection,
            COLORMAP_ALLOC_NONE as u8,
            colormap,
            rootwindow,
            visualid as u32
        );
        let values = [
            (CW_EVENT_MASK,
                EVENT_MASK_EXPOSURE
                | EVENT_MASK_KEY_PRESS
                | EVENT_MASK_KEY_RELEASE
                | EVENT_MASK_BUTTON_PRESS
                | EVENT_MASK_BUTTON_RELEASE
                | EVENT_MASK_POINTER_MOTION
                | EVENT_MASK_STRUCTURE_NOTIFY
            ),
            (CW_COLORMAP,colormap),
        ];
        let (window_width,window_height) = match config.window {
            WindowConfig::Standard => {
                (640,360)
            },
            WindowConfig::High => {
                (1280,720)
            }
        };
        create_window(
            &connection,
            depth as u8,
            window,
            rootwindow,
            0,0,window_width,window_height,
            0,
            WINDOW_CLASS_INPUT_OUTPUT as u16,
            visualid as u32,
            &values
        );
        unsafe {
            map_window(&connection,window);
            connection.flush();
            XSync(connection.get_raw_dpy(),False);
        }

        let protocol_set = [wm_delete_window];
        change_property(&connection,PROP_MODE_REPLACE as u8,window,wm_protocols,ATOM_ATOM,32,&protocol_set);

        let context = {
            let context_attribs: [c_int; 5] = [
                GLX_CONTEXT_MAJOR_VERSION_ARB as c_int,4,
                GLX_CONTEXT_MINOR_VERSION_ARB as c_int,5,
                0,
            ];
            let context = unsafe {
                glx_create_context_attribs(
                    connection.get_raw_dpy(),
                    fbconfig,
                    null_mut(),
                    True,
                    &context_attribs[0] as *const c_int
                )
            };
            connection.flush();
            unsafe { XSync(connection.get_raw_dpy(),False) };
            if context.is_null() {
                return Err(VideoError::Generic);
            }
            if unsafe { glXIsDirect(connection.get_raw_dpy(),context) } == 0 {
                return Err(VideoError::Generic);
            }
            unsafe { glXMakeCurrent(connection.get_raw_dpy(),window as XID,context) };
            gl::load_with(|symbol| load_function(&symbol));
            context
        };

        let (framebuffer_width,framebuffer_height) = match config.framebuffer {
            FramebufferConfig::Standard => { (640,360) },
            FramebufferConfig::Low => { (320,180) },
        };
        let framebuffer = match Framebuffer::new(framebuffer_width,framebuffer_height) {
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
        let cs = CString::new(title).unwrap();
        change_property(&self.connection,PROP_MODE_REPLACE as u8,self.window,ATOM_WM_NAME,ATOM_STRING,8,cs.as_bytes());
        self.connection.flush();
    }

    fn handle_event(&mut self,xcb_event: GenericEvent) -> Option<Event> {
        let r = xcb_event.response_type() & !0x80;
        match r {
            EXPOSE => {
                unsafe {
                    glXMakeCurrent(self.connection.get_raw_dpy(),self.window as u64,self.context);
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
                        gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid);
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
                    gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const GLvoid);
                    gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
                    gl::DisableVertexAttribArray(0);
                    gl::Flush();
                    glXSwapBuffers(self.connection.get_raw_dpy(),self.window as XID);
                }
            },
            KEY_PRESS => {
                let key_press: &KeyPressEvent = unsafe { cast_event(&xcb_event) };
                let k = key_press.detail() as u8;
                return Some(Event::KeyPress(k));
            },
            KEY_RELEASE => {
                let key_release: &KeyReleaseEvent = unsafe { cast_event(&xcb_event) };
                let k = key_release.detail() as u8;
                return Some(Event::KeyRelease(k));
            },
            BUTTON_PRESS => {
                let button_press: &ButtonPressEvent = unsafe { cast_event(&xcb_event) };
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
            BUTTON_RELEASE => {
                let button_release: &ButtonReleaseEvent = unsafe { cast_event(&xcb_event) };
                let x = button_release.event_x() as i32;
                let y = button_release.event_y() as i32;
                match button_release.detail() {
                    1 => { return Some(Event::MouseRelease(x,y,Button::Left)); },
                    2 => { return Some(Event::MouseRelease(x,y,Button::Middle)); },
                    3 => { return Some(Event::MouseRelease(x,y,Button::Right)); },
                    _ => { },
                }
            },
            MOTION_NOTIFY => {
                let motion_notify: &MotionNotifyEvent = unsafe { cast_event(&xcb_event) };
                let x = motion_notify.event_x() as i32;
                let y = motion_notify.event_y() as i32;
                return Some(Event::MouseMove(x,y));
            },
            CONFIGURE_NOTIFY => {
                let configure_notify: &ConfigureNotifyEvent = unsafe { cast_event(&xcb_event) };
                let width = configure_notify.width() as u32;
                let height = configure_notify.height() as u32;
                if (width != self.window_width) || (height != self.window_height) {
                    self.window_width = width;
                    self.window_height = height;
                    return Some(Event::Resize(width,height));
                }
            },
            CLIENT_MESSAGE => {
                let client_message : &ClientMessageEvent = unsafe { cast_event(&xcb_event) };
                let data = &client_message.data().data;
                let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);
                if atom == self.wm_delete_window {
                    return Some(Event::Close);
                }
            },
            _ => { },
        }
        None
    }

    pub fn poll_for_event(&mut self) -> Option<Event> {
        while let Some(xcb_event) = self.connection.poll_for_event() {
            if let Some(event) = self.handle_event(xcb_event) {
                return Some(event);
            }
        }
        None
    }

    pub fn wait_for_event(&mut self) -> Option<Event> {
        while let Some(xcb_event) = self.connection.wait_for_event() {
            if let Some(event) = self.handle_event(xcb_event) {
                return Some(event);
            }
        }
        None
    }
}

impl Drop for Video {
    fn drop(&mut self) {
        unsafe { glXMakeCurrent(self.connection.get_raw_dpy(),0,null_mut()); }
        unmap_window(&self.connection,self.window);
        destroy_window(&self.connection,self.window);
        unsafe { glXDestroyContext(self.connection.get_raw_dpy(),self.context); }
    }
}
