// G - Image test
// Desmond Germans, 2020

/*use g::Video;
use g::VideoConfig;
use g::WindowConfig;
use g::FramebufferConfig;
use g::Layer;
use std::fs::File;
use std::io::prelude::*;
use g::decode;
use g::Texture2D;
use g::Shader;
use g::SetUniform;
use g::Event;
use g::ARGB8;
use g::Texture2DUpload;
use std::ffi::c_void;
use g::OpenGLFormat;
use gl::types::GLenum;
use gl::types::GLuint;

#[derive(Copy,Clone)]
struct TileIndex {
    d: u32,
}

impl TileIndex {
    pub fn new_empty() -> TileIndex {
        TileIndex {
            d: 0x00000000,
        }
    }
}

impl OpenGLFormat for TileIndex {
    fn gl_internal_format() -> GLuint { gl::R32UI as GLuint }
    fn gl_format() -> GLenum { gl::RED_INTEGER }
    fn gl_type() -> GLenum { gl::UNSIGNED_INT }
}

struct Map {
    width: usize,
    height: usize,
    pub data: Box<[TileIndex]>,
}

impl Map {
    pub fn new(width: usize,height: usize) -> Map {
        Map {
            width: width,
            height: height,
            data: vec![TileIndex::new_empty(); width * height].into_boxed_slice(),
        }
    }
}

impl Texture2DUpload<Map> for Texture2D<TileIndex> {
    fn upload(&mut self,x: isize,y: isize,source: &Map) {
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D,0,x as i32,y as i32,source.width as i32,source.height as i32,TileIndex::gl_format(),TileIndex::gl_type(),source.data.as_ptr() as *const c_void);
        }
    }    
}

fn load_texture(name: &str) -> Texture2D<ARGB8> {
    let mut file = File::open(name).expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let image = decode::<ARGB8>(&buffer).expect("unable to decode");
    let mut texture = Texture2D::<ARGB8>::new(image.width as usize,image.height as usize);
    texture.upload(0,0,&image);
    texture
}

fn main() {
    let config = VideoConfig {
        window: WindowConfig { width: 1280,height: 720, },
        framebuffer: FramebufferConfig { width: 256,height: 144, },
    };
    let mut video = match Video::new(config) {
        Ok(video) => video,
        Err(_) => { panic!("Cannot open video."); },
    };
    video.set_window_title("Image Test");
    let layer = Layer::new(0,0,config.framebuffer.width,config.framebuffer.height).expect("cannot create layer");
    let tiles_texture = load_texture("try/8x8tiles.png");
    let mut map = Map::new(4,4);
    map.data[0] = TileIndex { d: 0x00000001, };
    map.data[1] = TileIndex { d: 0x00000001, };
    map.data[2] = TileIndex { d: 0x00000001, };
    map.data[3] = TileIndex { d: 0x00000000, };
    map.data[4] = TileIndex { d: 0x00000001, };
    map.data[5] = TileIndex { d: 0x00000003, };
    map.data[6] = TileIndex { d: 0x00000001, };
    map.data[7] = TileIndex { d: 0x00000000, };
    map.data[8] = TileIndex { d: 0x00000001, };
    map.data[9] = TileIndex { d: 0x00000001, };
    map.data[10] = TileIndex { d: 0x00000001, };
    map.data[11] = TileIndex { d: 0x00000000, };
    map.data[12] = TileIndex { d: 0x00000002, };
    map.data[13] = TileIndex { d: 0x00000002, };
    map.data[14] = TileIndex { d: 0x00000002, };
    map.data[15] = TileIndex { d: 0x00000002, };
    let mut map_texture = Texture2D::<TileIndex>::new(map.width,map.height);
    map_texture.upload(0,0,&map);

    let vs = r#"
        #version 420 core
        layout(location = 0) in vec2 v_pos;
        out vec2 f_tex;
        void main() {
            f_tex = vec2(v_pos.x,v_pos.y);
            gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
        }
    "#;
    let fs = r#"
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
            fs_output = vec4(d.x,d.y,d.z,1.0);
        }
    "#;
    let shader = Shader::new(vs,None,fs).expect("cannot create shader");
    unsafe {
        layer.bind();
        gl::ClearColor(1.0,1.0,0.0,1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        shader.bind();
        shader.set_uniform("map_texture",0);
        shader.set_uniform("atlas_texture",1);
        shader.set_uniform("offset",[0.125,0.125]);
        shader.set_uniform("tiles_per_pixel",[0.125f32,0.125f32]);
        shader.set_uniform("pixels_per_layer",[config.framebuffer.width as f32,config.framebuffer.height as f32]);
        shader.set_uniform("maps_per_tile",[0.25f32,0.25f32]);
        gl::ActiveTexture(gl::TEXTURE0 + 1);
        tiles_texture.bind();
        gl::ActiveTexture(gl::TEXTURE0 + 0);  // bind 0 last, for some obscure reason...
        map_texture.bind();
        gl::BindBuffer(gl::ARRAY_BUFFER,video.opengl.quad_vbo);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const gl::types::GLvoid);
        gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
        gl::DisableVertexAttribArray(0);
        gl::Flush();
        layer.unbind();
    }
    video.opengl.layers.push(layer);
    loop {
        let event = video.wait_for_event().expect("Event queue error.");
        match event {
            Event::Close => {
                return;
            },
            _ => { },
        }    
    }
}*/

fn main() {
}