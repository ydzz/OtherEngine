use crate::graphics::gfx_helper::{BufferState};
use crate::graphics::gfx_helper::{Vertex3};
use crate::graphics::mesh::{Mesh};
use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal::{ Backend,
               device::{Device}, 
               buffer, 
               adapter::{Adapter, MemoryType},
               memory as m
             };

pub struct MeshStore<B:gfx_hal::Backend> {
 pub quad:Rc<Mesh<B>>
}

impl<B> MeshStore<B>  where B: gfx_hal::Backend {
  pub fn new(device:Rc<RefCell<B::Device>>,memory_types: &[MemoryType]) -> Self {
    MeshStore {
      quad:Rc::new( create_quad_mesh(&device, memory_types))
    }
  }

  


}

fn create_quad_mesh<B:gfx_hal::Backend>(device:&Rc<RefCell<B::Device>>,memory_types: &[MemoryType]) -> Mesh<B> {
    let quad2d: [Vertex3; 6] = [ Vertex3 { a_pos: [-0.5, 0.5,0f32],  a_uv: [0.0, 1.0] },
                                 Vertex3 { a_pos: [0.5, 0.5,0f32],   a_uv: [1.0, 1.0] },
                                 Vertex3 { a_pos: [0.5, -0.5,0f32],  a_uv: [1.0, 0.0]},
                                 Vertex3 { a_pos: [-0.5, 0.5,0f32],  a_uv: [0.0, 1.0]},
                                 Vertex3 { a_pos: [0.5, -0.5,0f32],  a_uv: [1.0, 0.0]},
                                 Vertex3 { a_pos: [-0.5, -0.5,0f32], a_uv: [0.0, 0.0]}];
    let buffer = unsafe { BufferState::new::<Vertex3>(Rc::clone(&device),&quad2d, buffer::Usage::VERTEX,memory_types) };
    Mesh {vertex_count: 6, buffer : buffer }
  }