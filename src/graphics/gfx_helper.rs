extern crate gfx_backend_gl as back;
use std::mem::{size_of, ManuallyDrop};
use std::{fs, iter, ptr};
use gfx_hal::{ Backend,
               device::{Device}, 
               buffer, 
               adapter::{Adapter, MemoryType},
               memory as m
             };
use std::rc::{Rc};
use std::cell::{RefCell};
#[derive(Debug, Clone, Copy)]
pub struct Vertex2 {
  pub a_pos: [f32; 2],
  pub  a_uv:  [f32; 2]
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex3 {
    a_pos: [f32; 3],
    a_uv:  [f32; 2]
}

pub struct BufferState<B: Backend> {
  memory: Option<B::Memory>,
  buffer: Option<B::Buffer>,
  size: u64,
  device:Rc<RefCell<B::Device>>
}

impl<B: Backend> BufferState<B> {
  pub fn get_buffer(&self) -> &B::Buffer {
    self.buffer.as_ref().unwrap()
  }

  pub unsafe fn new<T>(device_ptr:Rc<RefCell<B::Device>>,
                   data_source: &[T],
                   usage: buffer::Usage,
                   memory_types: &[MemoryType]) -> Self  where T: Copy {
    let memory: B::Memory;
    let mut buffer: B::Buffer;
    let size: u64;
    let stride = size_of::<T>();
    let upload_size = data_source.len() * stride;
    {
     let device_ref = device_ptr.borrow();
     buffer = device_ref.create_buffer(upload_size as u64, usage).unwrap();
     let mem_req = device_ref.get_buffer_requirements(&buffer);
    
     let upload_type = memory_types
                .iter()
                .enumerate()
                .position(|(id, mem_type)| {
                    mem_req.type_mask & (1 << id) != 0
                        && mem_type.properties.contains(m::Properties::CPU_VISIBLE | m::Properties::COHERENT)
                }).unwrap().into();

     memory = device_ref.allocate_memory(upload_type, mem_req.size).unwrap();
     device_ref.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
     size = mem_req.size;
     let mapping = device_ref.map_memory(&memory, 0 .. size).unwrap();
     ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
     device_ref.unmap_memory(&memory);
    }
    BufferState {
       memory: Some(memory),
       buffer: Some(buffer),
       device: device_ptr,
       size
    }
  }
}