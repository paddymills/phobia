
use std::path::PathBuf;

use glium::{
    Display,
    glutin::surface::WindowSurface,
    vertex::VertexBufferAny
};

use obj::{Obj, ObjData};
use crate::formats::wavefront;

#[derive(Debug)]
pub struct Model {
    pub geometry: Obj,

    pub vb: Option<VertexBufferAny>
}

impl Model {
    pub fn new() -> Self {
        Self {
            geometry: Obj { data: ObjData::default(), path: PathBuf::new() },
            vb: None
        }
    }

    pub fn load(&mut self, path: PathBuf) {
        self.geometry = Obj::load(path).unwrap();
    }

    pub fn vertex_buffer(&mut self, display: &Display<WindowSurface>) -> &VertexBufferAny {
        if let None = self.vb {
            self.vb = Some( wavefront::load(display, &self.geometry) );
        }

        // previous lines ensure this will not panic
        self.vb.as_ref().unwrap()
    }
}