use crate::graphics::gfx_helper::{BufferState};
use crate::graphics::gfx_helper::{Vertex2};
use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal::{ Backend,
               device::{Device}, 
               buffer, 
               adapter::{Adapter, MemoryType},
               memory as m
             };

pub struct MeshStore<B:gfx_hal::Backend> {
  quad2d:BufferState<B>
}

impl<B> MeshStore<B>  where B: gfx_hal::Backend {
  pub fn new(device:Rc<RefCell<B::Device>>,memory_types: &[MemoryType]) -> Self {
    MeshStore {
      quad2d:create_quad2d_mesh(&device, memory_types)
    }
  }

  

  pub fn get_quad2d(&self) -> &BufferState<B> {
    &self.quad2d
  }
}

fn create_quad2d_mesh<B:gfx_hal::Backend>(device:&Rc<RefCell<B::Device>>,memory_types: &[MemoryType]) -> BufferState<B> {
    let quad2d: [Vertex2; 6] = [ Vertex2 { a_pos: [-0.5, 0.5], a_uv: [0.0, 1.0] },
                                 Vertex2 { a_pos: [0.5, 0.5], a_uv: [1.0, 1.0] },
                                 Vertex2 {  a_pos: [0.5, -0.5],  a_uv: [1.0, 0.0]},
                                 Vertex2 { a_pos: [-0.5, 0.5], a_uv: [0.0, 1.0]},
                                 Vertex2 {a_pos: [0.5, -0.5],a_uv: [1.0, 0.0]},
                                 Vertex2 {a_pos: [-0.5, -0.5],a_uv: [0.0, 0.0]}];
    unsafe { BufferState::new::<Vertex2>(Rc::clone(&device),&quad2d, buffer::Usage::VERTEX,memory_types) }
  }