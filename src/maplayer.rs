// G - OpenGL - Layer
// Desmond Germans, 2020

use crate::*;
use std::{
    rc::Rc,
    cell::{
        Cell,
        RefCell,
    },
};

pub struct MapLayer {
    pub(crate) engine: Rc<Engine>,
    pub(crate) framebuffer: Rc<e::Framebuffer>,
    pub(crate) atlas_texture: RefCell<Rc<e::Texture2D<pixel::ARGB8>>>,
    pub(crate) map_texture: RefCell<Rc<e::Texture2D<u32>>>,
    pub(crate) offset: Cell<Vec2<f32>>,
}

impl MapLayer {
    pub fn new(engine: &Rc<Engine>) -> Result<MapLayer,EngineError> {
        let framebuffer = Rc::new(e::Framebuffer::new(&engine.graphics,engine.framebuffer.size).expect("MapLayer::new: Unable to create framebuffer."));
        let atlas_texture = Rc::new(e::Texture2D::new(&engine.graphics,vec2!(64,64)).expect("MapLayer::new: Unable to create atlas texture."));
        let map_texture = Rc::new(e::Texture2D::new(&engine.graphics,vec2!(256,256)).expect("MapLayer::new: Unable to create map texture."));
        Ok(MapLayer {
            engine: Rc::clone(engine),
            framebuffer: framebuffer,
            atlas_texture: RefCell::new(atlas_texture),
            map_texture: RefCell::new(map_texture),
            offset: Cell::new(vec2!(0.0,0.0)),
        })
    }

    pub fn set_atlas_from_mat(&self,image: Mat<pixel::ARGB8>) {
        (*self.atlas_texture.borrow_mut()).load(vec2!(0,0),&image);
    }

    pub fn set_map_from_mat(&self,image: Mat<u32>) {
        (*self.map_texture.borrow_mut()).load(vec2!(0,0),&image);
    }
}

impl Layer for MapLayer {
    fn framebuffer(&self) -> &gpu::Framebuffer {
        &*self.framebuffer
    }

    fn render(&self) {
        self.engine.graphics.bind_target(&*self.framebuffer);
        self.engine.graphics.bind_shader(&self.engine.map_shader);
        self.engine.graphics.bind_texture(0,&**(self.atlas_texture.borrow()));
        self.engine.graphics.set_uniform("atlas_texture",0);
        self.engine.graphics.bind_texture(1,&**(self.map_texture.borrow()));
        self.engine.graphics.set_uniform("map_texture",1);
        self.engine.graphics.set_uniform("offset",self.offset.get());
        self.engine.graphics.set_uniform("tiles_per_pixel",vec2!(0.125,0.125));
        self.engine.graphics.set_uniform("pixels_per_layer",vec2!(self.framebuffer.size.x as f32,self.framebuffer.size.y as f32));
        self.engine.graphics.set_uniform("maps_per_tile",vec2!(0.25,0.25));
        self.engine.graphics.bind_vertexbuffer(&self.engine.quad_vertexbuffer);
        self.engine.graphics.draw_triangle_fan(4);            
    }
}
