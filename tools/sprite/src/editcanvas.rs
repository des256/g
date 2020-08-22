// G Sprite Editor - Editor Canvas
// by Desmond Germans, 2020

use e::*;
use std::{
    rc::Rc,
    cell::Cell,
};

use crate::document::*;

const SCALE_GROW: f32 = 1.1;

pub struct EditCanvas {
    ui: Rc<ui::UI>,
    background_grid_shader: gpu::Shader,
    layer_shader: gpu::Shader,
    pixel_grid_shader: gpu::Shader,
    selection_shader: gpu::Shader,
    quad_point: gpu::VertexBuffer::<Vec4<f32>>,
    document: Rc<Document>,
    mouse: Cell<Vec2<i32>>,
}

impl EditCanvas {
    pub fn new(ui: &Rc<ui::UI>,document: &Rc<Document>) -> Result<EditCanvas,SystemError> {

        // vertex shader
        let vs = r#"
            #version 420 core

            layout(location = 0) in vec4 ir;

            void main() {
                gl_Position = ir;
            }
        "#;

        // geometry shader
        let gs = r#"
            #version 420 core

            uniform vec2 canvas_size;  // size of the output window
            uniform vec4 space;        // rectangle to draw
            uniform vec2 offset;       // offset for texture sampling
            uniform vec2 scale;        // scale for texture sampling
            
            layout(points) in;
            layout(triangle_strip,max_vertices = 4) out;

            out Vertex {
                vec2 t;
            } gs_out;

            void main() {

                vec4 r = gl_in[0].gl_Position;

                vec2 ooc = vec2(1.0 / canvas_size.x,1.0 / canvas_size.y);
                vec2 spc = vec2(space.z * ooc.x,space.w * ooc.y);

                vec4 pn = vec4(
                    -1.0 + 2.0 * space.x * ooc.x,
                    1.0 - 2.0 * space.y * ooc.y,
                    2.0 * spc.x,
                    -2.0 * spc.y
                );

                vec4 pt = vec4(
                    offset.x,
                    offset.y,
                    space.z / scale.x,
                    space.w / scale.y
                );

                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                gs_out.t = vec2(pt.x,pt.y);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y,0.0,1.0);
                gs_out.t = vec2(pt.x + pt.z,pt.y);
                EmitVertex();

                gl_Position = vec4(pn.x,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(pt.x,pt.y + pt.w);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(pt.x + pt.z,pt.y + pt.w);
                EmitVertex();

                EndPrimitive();
            }
        "#;

        // create background grid shader
        let background_grid_fs = r#"
            #version 420 core

            uniform vec2 background_grid_size;

            in Vertex {
                vec2 t;
            } fs_in;
            
            out vec4 o;
            
            void main() {
                ivec2 t = ivec2(int(round(fs_in.t.x / background_grid_size.x)),int(round(fs_in.t.y / background_grid_size.y)));
                bool b = false;
                if ((t.x & 1) == 1) {
                    b = true;
                }
                if ((t.y & 1) == 1) {
                    b = !b;
                }
                o = b?vec4(0.3,0.3,0.3,1.0):vec4(0.5,0.5,0.5,1.0);
            }
        "#;
        let background_grid_shader = gpu::Shader::new(&ui.graphics,vs,Some(gs),background_grid_fs).expect("Unable to create background grid shader.");

        // create layer shader
        let layer_fs = r#"
            #version 420 core

            uniform vec2 image_size;

            uniform sampler2D layer_texture;

            in Vertex {
                vec2 t;
            } fs_in;

            out vec4 o;

            void main() {
                vec2 tc = vec2(fs_in.t.x / image_size.x,fs_in.t.y / image_size.y);
                o = texture(layer_texture,tc);
            }
        "#;
        let layer_shader = gpu::Shader::new(&ui.graphics,vs,Some(gs),layer_fs).expect("Unable to create layer shader.");

        // create selection shader
        let selection_fs = r#"
            #version 420 core

            in Vertex {
                vec2 t;
            } fs_in;

            out vec4 o;

            void main() {
                o = vec4(1.0,0.0,0.0,1.0);
            }
        "#;
        let selection_shader = gpu::Shader::new(&ui.graphics,vs,Some(gs),selection_fs).expect("Unable to create selection shader.");

        // create pixel grid shader
        let pixel_grid_fs = r#"
            #version 420 core

            in Vertex {
                vec2 t;
            } fs_in;

            out vec4 o;

            void main() {
                vec2 b = vec2(512.0,512.0);
                vec2 tc = fs_in.t;
                vec2 rtc = vec2(round(tc.x),round(tc.y));
                vec2 d = vec2(tc.x - rtc.x,tc.y - rtc.y);
                vec2 a = vec2(exp(-(d.x * d.x * b.x)),exp(-(d.y * d.y * b.y)));
                o = vec4(0.0,0.0,0.0,a.x + a.y);
            }
        "#;
        let pixel_grid_shader = gpu::Shader::new(&ui.graphics,vs,Some(gs),pixel_grid_fs).expect("Unable to create pixel grid shader.");

        let quad_point = gpu::VertexBuffer::<Vec4<f32>>::new_from_vec(&ui.graphics,&vec![vec4!(0.0,0.0,1.0,1.0)]).expect("Unable to create vertexbuffer.");

        Ok(EditCanvas {
            ui: Rc::clone(ui),
            background_grid_shader: background_grid_shader,
            layer_shader: layer_shader,
            selection_shader: selection_shader,
            pixel_grid_shader: pixel_grid_shader,
            quad_point: quad_point,
            document: Rc::clone(document),
            mouse: Cell::new(vec2!(0,0)),
        })
    }

    pub fn draw_background_grid(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        let background_grid_size = self.document.background_grid_size.get();
        self.ui.graphics.bind_shader(&self.background_grid_shader);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",offset);
        self.ui.graphics.set_uniform("scale",scale);
        self.ui.graphics.set_uniform("background_grid_size",vec2!(background_grid_size.x as f32,background_grid_size.y as f32));
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        self.ui.graphics.draw_points(1);
    }

    pub fn draw_layer(&self,canvas_size: Vec2<i32>,space: Rect<i32>,layer: &Layer) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        self.ui.graphics.bind_shader(&self.layer_shader);
        self.ui.graphics.bind_texture(0,&layer.texture);
        self.ui.graphics.set_uniform("layer_texture",0);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",offset);
        self.ui.graphics.set_uniform("scale",scale);
        self.ui.graphics.set_uniform("image_size",vec2!(layer.texture.size.x as f32,layer.texture.size.y as f32));
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        self.ui.graphics.draw_points(1);
    }

    pub fn draw_selection(&self,canvas_size: Vec2<i32>,space: Rect<i32>,selection: &Selection) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        self.ui.graphics.bind_shader(&self.selection_shader);
        self.ui.graphics.bind_texture(0,&selection.texture);
        self.ui.graphics.set_uniform("selection_texture",0);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",offset);
        self.ui.graphics.set_uniform("scale",scale);
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        //self.ui.graphics.draw_points(1);
    }

    pub fn draw_pixel_grid(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        self.ui.graphics.bind_shader(&self.pixel_grid_shader);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",offset);
        self.ui.graphics.set_uniform("scale",scale);
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        self.ui.graphics.draw_points(1);
    }
}

impl ui::Widget for EditCanvas {
    fn measure(&self) -> Vec2<i32> {
        vec2!(1280,640)
    }

    fn handle(&self,event: &Event,_space: Rect<i32>) {
        match event {
            Event::MouseWheel(w) => {
                match w {
                    Wheel::Up => {
                        let mut offset = self.document.offset.get();
                        let mut scale = self.document.scale.get();
                        let mouse = self.mouse.get();
                        offset += vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                        scale *= SCALE_GROW;
                        offset -= vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                        self.document.offset.set(offset);
                        self.document.scale.set(scale);
                    },
                    Wheel::Down => {
                        let mut offset = self.document.offset.get();
                        let mut scale = self.document.scale.get();
                        let mouse = self.mouse.get();
                        offset += vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                        scale /= SCALE_GROW;
                        offset -= vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                        self.document.offset.set(offset);
                        self.document.scale.set(scale);
                    },
                    _ => { },
                }
            },
            Event::MouseMove(p) => {
                self.mouse.set(*p);
            }
            _ => { },
        }
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let scale = self.document.scale.get();
        self.ui.graphics.set_blend(gpu::BlendMode::Replace);
        self.draw_background_grid(canvas_size,space);
        for layer in self.document.layers.iter() {
            self.ui.graphics.set_blend(layer.blend_mode);
            self.draw_layer(canvas_size,space,&layer);
        }
        self.ui.graphics.set_blend(gpu::BlendMode::Over);
        self.draw_selection(canvas_size,space,&self.document.selection);
        if (scale.x > 8.0) || (scale.y > 8.0) {
            self.draw_pixel_grid(canvas_size,space);
        }
    }
}
