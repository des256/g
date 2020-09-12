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
    core: ui::Core<Box<dyn ui::Widget>>,
    background_grid_shader: gpu::Shader,
    layer_shader: gpu::Shader,
    pixel_grid_shader: gpu::Shader,
    selection_shader: gpu::Shader,
    document: Rc<Document>,
    mouse: Cell<Vec2<i32>>,
}

impl EditCanvas {
    pub fn new(state: &Rc<ui::UIState>,document: &Rc<Document>) -> Result<EditCanvas,SystemError> {

        // vertex shader
        let vs = r#"
            #version 420 core
            
            layout(location = 0) in vec2 i_p;
            
            uniform vec2 tows;
            uniform vec4 rect;
            uniform vec2 offset;
            uniform vec2 scale;

            out Vertex {
                vec2 t;
            } vs_out;
            
            void main() {
                gl_Position = vec4(
                    -1.0 + tows.x * (rect.x + i_p.x * rect.z),
                    1.0 - tows.y * (rect.y + i_p.y * rect.w),
                    0.0,
                    1.0
                );
                vs_out.t = vec2(
                    offset.x + i_p.x * rect.z / scale.x,
                    offset.y + i_p.y * rect.w / scale.y
                );
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
        let background_grid_shader = gpu::Shader::new(&state.graphics,vs,None,background_grid_fs).expect("Unable to create background grid shader.");

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
        let layer_shader = gpu::Shader::new(&state.graphics,vs,None,layer_fs).expect("Unable to create layer shader.");

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
        let selection_shader = gpu::Shader::new(&state.graphics,vs,None,selection_fs).expect("Unable to create selection shader.");

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
        let pixel_grid_shader = gpu::Shader::new(&state.graphics,vs,None,pixel_grid_fs).expect("Unable to create pixel grid shader.");

        Ok(EditCanvas {
            core: ui::Core::new(state),
            background_grid_shader: background_grid_shader,
            layer_shader: layer_shader,
            selection_shader: selection_shader,
            pixel_grid_shader: pixel_grid_shader,
            document: Rc::clone(document),
            mouse: Cell::new(vec2!(0,0)),
        })
    }

    pub fn draw_background_grid(&self,context: Vec2<i32>) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        let background_grid_size = self.document.background_grid_size.get();
        self.core.state.graphics.bind_shader(&self.background_grid_shader);
        self.core.state.graphics.bind_vertexbuffer(&self.core.state.rect_vb);
        self.core.state.graphics.set_uniform("tows",self.core.state.two_over_current_window_size.get());
        let r = self.core.r.get();
        self.core.state.graphics.set_uniform("rect",vec4!((r.o.x + context.x) as f32,(r.o.y + context.y) as f32,r.s.x as f32,r.s.y as f32));
        self.core.state.graphics.set_uniform("offset",offset);
        self.core.state.graphics.set_uniform("scale",scale);
        self.core.state.graphics.set_uniform("background_grid_size",vec2!(background_grid_size.x as f32,background_grid_size.y as f32));
        self.core.state.graphics.draw_instanced_triangle_fan(4,1);
    }

    pub fn draw_layer(&self,context: Vec2<i32>,layer: &Layer) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        self.core.state.graphics.bind_shader(&self.layer_shader);
        self.core.state.graphics.bind_vertexbuffer(&self.core.state.rect_vb);
        self.core.state.graphics.bind_texture(0,&layer.texture);
        self.core.state.graphics.set_uniform("layer_texture",0);
        self.core.state.graphics.set_uniform("tows",self.core.state.two_over_current_window_size.get());
        let r = self.core.r.get();
        self.core.state.graphics.set_uniform("rect",vec4!((r.o.x + context.x) as f32,(r.o.y + context.y) as f32,r.s.x as f32,r.s.y as f32));
        self.core.state.graphics.set_uniform("offset",offset);
        self.core.state.graphics.set_uniform("scale",scale);
        self.core.state.graphics.set_uniform("image_size",vec2!(layer.texture.size.x as f32,layer.texture.size.y as f32));
        self.core.state.graphics.draw_instanced_triangle_fan(4,1);
    }

    pub fn draw_selection(&self,context: Vec2<i32>,selection: &Selection) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        self.core.state.graphics.bind_shader(&self.selection_shader);
        self.core.state.graphics.bind_vertexbuffer(&self.core.state.rect_vb);
        self.core.state.graphics.bind_texture(0,&selection.texture);
        self.core.state.graphics.set_uniform("selection_texture",0);
        self.core.state.graphics.set_uniform("tows",self.core.state.two_over_current_window_size.get());
        let r = self.core.r.get();
        self.core.state.graphics.set_uniform("rect",vec4!((r.o.x + context.x) as f32,(r.o.y + context.y) as f32,r.s.x as f32,r.s.y as f32));
        self.core.state.graphics.set_uniform("offset",offset);
        self.core.state.graphics.set_uniform("scale",scale);
        //self.core.state.graphics.draw_instanced_triangle_fan(4,1);
    }

    pub fn draw_pixel_grid(&self,context: Vec2<i32>) {
        let offset = self.document.offset.get();
        let scale = self.document.scale.get();
        self.core.state.graphics.bind_shader(&self.pixel_grid_shader);
        self.core.state.graphics.bind_vertexbuffer(&self.core.state.rect_vb);
        self.core.state.graphics.set_uniform("tows",self.core.state.two_over_current_window_size.get());
        let r = self.core.r.get();
        self.core.state.graphics.set_uniform("rect",vec4!((r.o.x + context.x) as f32,(r.o.y + context.y) as f32,r.s.x as f32,r.s.y as f32));
        self.core.state.graphics.set_uniform("offset",offset);
        self.core.state.graphics.set_uniform("scale",scale);
        self.core.state.graphics.draw_instanced_triangle_fan(4,1);
    }
}

impl ui::Widget for EditCanvas {
    fn get_rect(&self) -> Rect<i32> {
        self.core.r.get()
    }

    fn set_rect(&self,r: Rect<i32>) {
        self.core.r.set(r);
    }

    fn calc_min_size(&self) -> Vec2<i32> {
        vec2!(1280,640)
    }

    fn draw(&self,context: Vec2<i32>) {
        let scale = self.document.scale.get();
        self.core.state.graphics.set_blend(gpu::BlendMode::Replace);
        self.draw_background_grid(context);
        for layer in self.document.layers.iter() {
            self.core.state.graphics.set_blend(layer.blend_mode);
            self.draw_layer(context,&layer);
        }
        self.core.state.graphics.set_blend(gpu::BlendMode::Over);
        self.draw_selection(context,&self.document.selection);
        if (scale.x > 8.0) || (scale.y > 8.0) {
            self.draw_pixel_grid(context);
        }
    }

    fn handle_mouse_press(&self,_p: Vec2<i32>,_b: MouseButton) {

    }

    fn handle_mouse_release(&self,_p: Vec2<i32>,_b: MouseButton) {

    }

    fn handle_mouse_move(&self,p: Vec2<i32>) -> bool {
        self.mouse.set(p);
        false
    }

    fn handle_mouse_wheel(&self,w: MouseWheel) {
        match w {
            MouseWheel::Up => {
                let mut offset = self.document.offset.get();
                let mut scale = self.document.scale.get();
                let mouse = self.mouse.get();
                offset += vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                scale *= SCALE_GROW;
                offset -= vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                self.document.offset.set(offset);
                self.document.scale.set(scale);
                self.core.state.invalidate();
            },
            MouseWheel::Down => {
                let mut offset = self.document.offset.get();
                let mut scale = self.document.scale.get();
                let mouse = self.mouse.get();
                offset += vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                scale /= SCALE_GROW;
                offset -= vec2!((mouse.x as f32) / scale.x,(mouse.y as f32) / scale.y);
                self.document.offset.set(offset);
                self.document.scale.set(scale);
                self.core.state.invalidate();
            },
            _ => { },
        }
    }
}
