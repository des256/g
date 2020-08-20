// G Sprite Editor - Editor Canvas
// by Desmond Germans, 2020

use e::*;
use std::rc::Rc;
use gl::types::{
    GLuint,
    GLvoid,
};

use crate::document::*;
use crate::layer::*;
use crate::selection::*;
use crate::pixelgrid::*;

pub struct EditCanvas {
    ui: Rc<ui::UI>,
    layer_shader: gpu::Shader,
    pixel_grid_shader: gpu::Shader,
    selection_shader: gpu::Shader,
    quad_point: gpu::VertexBuffer::<Vec4<f32>>,
    document: Rc<Document>,
}

impl EditCanvas {
    pub fn new(ui: &Rc<ui::UI>,document: &Rc<Document>) -> Result<EditCanvas,SystemError> {

        // create layer shader
        let layer_vs = r#"
            #version 420 core

            layout(location = 0) in vec4 ir;

            void main() {
                gl_Position = ir;
            }
        "#;
        let layer_gs = r#"
            #version 420 core

            uniform vec2 canvas_size;
            uniform vec4 space;
            uniform vec2 offset;
            uniform vec2 scale;
            
            layout(points) in;
            layout(triangle_strip,max_vertices = 4) out;

            void main() {

                vec4 r = gl_in[0].gl_Position;

                vec4 pn = vec4(
                    -1.0 + 2.0 * r.x / canvas_size.x,
                    1.0 - 2.0 * r.y / canvas_size.y,
                    2.0 * r.z / canvas_size.x,
                    -2.0 * r.w / canvas_size.y
                );

                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y,0.0,1.0);
                EmitVertex();

                gl_Position = vec4(pn.x,pn.y + pn.w,0.0,1.0);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y + pn.w,0.0,1.0);
                EmitVertex();

                EndPrimitive();
            }
        "#;

        let layer_fs = r#"
            #version 420 core

            out vec4 o;

            void main() {
                o = vec4(1.0,0.0,0.0,1.0);
            }
        "#;
        let layer_shader = gpu::Shader::new(&ui.graphics,layer_vs,Some(layer_gs),layer_fs).expect("what?");

        // create selection shader
        let selection_vs = r#"
            #version 420 core

            layout(location = 0) in vec4 ir;

            void main() {
                gl_Position = ir;
            }
        "#;
        let selection_gs = r#"
            #version 420 core

            uniform vec2 canvas_size;
            uniform vec4 space;
            uniform vec2 offset;
            uniform vec2 scale;
            
            layout(points) in;
            layout(triangle_strip,max_vertices = 4) out;

            void main() {

                vec4 r = gl_in[0].gl_Position;
                vec4 t = gs_in[0].t;

                vec4 pn = vec4(
                    -1.0 + 2.0 * r.x / canvas_size.x,
                    1.0 - 2.0 * r.y / canvas_size.y,
                    2.0 * r.z / canvas_size.x,
                    -2.0 * r.w / canvas_size.y
                );

                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                gs_out.t = vec2(t.x,t.y);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y,0.0,1.0);
                gs_out.t = vec2(t.x + t.z,t.y);
                EmitVertex();

                gl_Position = vec4(pn.x,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(t.x,t.y + t.w);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y + pn.w,0.0,1.0);
                gs_out.t = vec2(t.x + t.z,t.y + t.w);
                EmitVertex();

                EndPrimitive();
            }
        "#;

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
        let selection_shader = gpu::Shader::new(&ui.graphics,selection_vs,Some(selection_gs),selection_fs).expect("what?");

        // create pixel grid shader
        let pixel_grid_vs = r#"
            #version 420 core

            layout(location = 0) in vec4 ir;

            void main() {
                gl_Position = ir;
            }
        "#;
        let pixel_grid_gs = r#"
            #version 420 core

            uniform vec2 canvas_size;
            uniform vec4 space;
            uniform vec2 offset;
            uniform vec2 scale;
            
            layout(points) in;
            layout(triangle_strip,max_vertices = 4) out;

            in Rect {
            } gs_in[];

            out Vertex {
            } gs_out;

            void main() {

                vec4 r = gl_in[0].gl_Position;

                vec4 pn = vec4(
                    -1.0 + 2.0 * r.x / canvas_size.x,
                    1.0 - 2.0 * r.y / canvas_size.y,
                    2.0 * r.z / canvas_size.x,
                    -2.0 * r.w / canvas_size.y
                );

                gl_Position = vec4(pn.x,pn.y,0.0,1.0);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y,0.0,1.0);
                EmitVertex();

                gl_Position = vec4(pn.x,pn.y + pn.w,0.0,1.0);
                EmitVertex();

                gl_Position = vec4(pn.x + pn.z,pn.y + pn.w,0.0,1.0);
                EmitVertex();

                EndPrimitive();
            }
        "#;

        let pixel_grid_fs = r#"
            #version 420 core

            in Vertex {
            } fs_in;

            out vec4 o;

            void main() {
                o = vec4(1.0,0.0,0.0,1.0);
            }
        "#;
        let pixel_grid_shader = gpu::Shader::new(&ui.graphics,pixel_grid_vs,Some(pixel_grid_gs),pixel_grid_fs).expect("what?");

        let quad_point = gpu::VertexBuffer::<Vec4<f32>>::new_from_vec(&ui.graphics,&vec![vec4!(0.0,0.0,1.0,1.0)]).expect("Unable to create vertexbuffer.");

        Ok(EditCanvas {
            ui: Rc::clone(ui),
            layer_shader: layer_shader,
            selection_shader: selection_shader,
            pixel_grid_shader: pixel_grid_shader,
            quad_point: quad_point,
            document: Rc::clone(document),
        })
    }

    pub fn draw_layer(&self,canvas_size: Vec2<i32>,space: Rect<i32>,offset: Vec2<i32>,scale: Vec2<f32>,layer: &Layer) {
        self.ui.graphics.bind_shader(&self.layer_shader);
        self.ui.graphics.bind_texture(0,&layer.texture);
        self.ui.graphics.set_uniform("layer_texture",0);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",vec2!(offset.x as f32,offset.y as f32));
        self.ui.graphics.set_uniform("scale",vec2!(scale.x as f32,scale.y as f32));
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        self.ui.graphics.draw_points(1);
    }

    pub fn draw_selection(&self,canvas_size: Vec2<i32>,space: Rect<i32>,offset: Vec2<i32>,scale: Vec2<f32>,selection: &Selection) {
        self.ui.graphics.bind_shader(&self.selection_shader);
        self.ui.graphics.bind_texture(0,&selection.texture);
        self.ui.graphics.set_uniform("layer_texture",0);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",vec2!(offset.x as f32,offset.y as f32));
        self.ui.graphics.set_uniform("scale",vec2!(scale.x as f32,scale.y as f32));
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        self.ui.graphics.draw_points(1);
    }

    pub fn draw_pixel_grid(&self,canvas_size: Vec2<i32>,space: Rect<i32>,offset: Vec2<i32>,scale: Vec2<f32>,pixel_grid: &PixelGrid) {
        self.ui.graphics.bind_shader(&self.pixel_grid_shader);
        self.ui.graphics.set_uniform("canvas_size",vec2!(canvas_size.x as f32,canvas_size.y as f32));
        self.ui.graphics.set_uniform("space",vec4!(space.o.x as f32,space.o.y as f32,space.s.x as f32,space.s.y as f32));
        self.ui.graphics.set_uniform("offset",vec2!(offset.x as f32,offset.y as f32));
        self.ui.graphics.set_uniform("scale",vec2!(scale.x as f32,scale.y as f32));
        self.ui.graphics.bind_vertexbuffer(&self.quad_point);
        self.ui.graphics.draw_points(1);
    }
}

impl ui::Widget for EditCanvas {
    fn measure(&self) -> Vec2<i32> {
        vec2!(0,0)
    }

    fn handle(&self,event: &Event,_space: Rect<i32>) {
    }

    fn draw(&self,canvas_size: Vec2<i32>,space: Rect<i32>) {
        let offset: Vec2<i32> = vec2!(0,0);
        let scale: Vec2<f32> = vec2!(1.0,1.0);
        // TODO: draw grid?
        for layer in self.document.layers.iter() {
            self.ui.graphics.set_blend(layer.blend_mode);
            self.draw_layer(canvas_size,space,offset,scale,&layer);
        }
        self.ui.graphics.set_blend(gpu::BlendMode::Over);
        self.draw_selection(canvas_size,space,offset,scale,&self.document.selection);
        self.draw_pixel_grid(canvas_size,space,offset,scale,&self.document.pixel_grid);
    }
}
