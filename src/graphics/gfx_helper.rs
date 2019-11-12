extern crate gfx_backend_gl as back;
use std::mem::{size_of, ManuallyDrop};
use std::{iter, ptr};
use image::GenericImageView;
use gfx_hal::{ Backend,
               device::{Device}, 
               buffer, 
               adapter::{Adapter, MemoryType},
               memory as m,
               pso,
               DescriptorPool
             };
use std::rc::{Rc};
use std::cell::{RefCell};
#[derive(Debug, Clone, Copy)]
pub struct Vertex2 {
  pub  a_pos: [f32; 2],
  pub  a_uv:  [f32; 2]
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex3 {
  pub  a_pos: [f32; 3],
  pub  a_uv:  [f32; 2]
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

  pub unsafe fn new_texture(device:&Rc<RefCell<B::Device>>,dynimage:&image::DynamicImage,memory_types: &[MemoryType]) -> (BufferState<B>,u32) {
    let (width,height) = dynimage.dimensions();
    let row_alignment_mask = 0;
    let image_stride = 4usize;
    let row_pitch = (width * image_stride as u32 + row_alignment_mask) & !row_alignment_mask;
    let upload_size = (height * row_pitch) as u64;

    let mut image_upload_buffer = device.borrow_mut().create_buffer(upload_size, buffer::Usage::TRANSFER_SRC).unwrap();
    let image_mem_reqs = device.borrow().get_buffer_requirements(&image_upload_buffer);
    let upload_type = memory_types.iter().enumerate().position(|(id, mem_type)| {
                              image_mem_reqs.type_mask & (1 << id) != 0 && mem_type.properties.contains(m::Properties::CPU_VISIBLE)
                         }).unwrap().into();
    let image_upload_memory = device.borrow().allocate_memory(upload_type, image_mem_reqs.size).unwrap();
    device.borrow_mut().bind_buffer_memory(&image_upload_memory, 0, &mut image_upload_buffer).unwrap();
    
    let mut data_target = device.borrow().acquire_mapping_writer(&image_upload_memory,0..image_mem_reqs.size).unwrap();
    let img = &dynimage.to_rgba();
    println!("{:?},len:{}",img.dimensions(),img.len());
    
    for y in 0 .. height as usize {
      let data_source_slice = &(**img)[y * (width as usize) * image_stride .. (y + 1) * (width as usize) * image_stride];
      let dest_base = y * row_pitch as usize;
      data_target[dest_base .. dest_base + data_source_slice.len()].copy_from_slice(data_source_slice);
    }
    device.borrow_mut().release_mapping_writer(data_target).unwrap();

    (BufferState {
      memory : Some(image_upload_memory),
      buffer : Some(image_upload_buffer),
      size : upload_size,
      device : Rc::clone(device)
    },row_pitch)
  }

  pub fn update<T>(&mut self, offset: u64, data_source: &[T]) where T : Copy {
    let stride = size_of::<T>();
    let upload_size = data_source.len() * stride;
    assert!(offset + upload_size as u64 <= self.size);
    let memory = self.memory.as_ref().unwrap();
    unsafe {
      let mapping = self.device.borrow().map_memory(memory, offset .. self.size).unwrap();
      ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
      self.device.borrow().unmap_memory(memory);
    }
  }
}

pub struct DescSetLayout<B: Backend> {
  layout: Option<B::DescriptorSetLayout>,
  pool: Option<B::DescriptorPool>,
  device: Rc<RefCell<B::Device>>
}

impl<B>  DescSetLayout<B> where B:Backend {
  pub  fn new(device:Rc<RefCell<B::Device>>, bindings: Vec<pso::DescriptorSetLayoutBinding>) -> Self {
    unsafe {
      let pools:Vec<pso::DescriptorRangeDesc> = bindings.iter().map(|binding| pso::DescriptorRangeDesc {
        ty : binding.ty,
        count : 1
      }).collect();
      let desc_set_layout = device.borrow().create_descriptor_set_layout(bindings, &[]).ok();
      let pool = device.borrow().create_descriptor_pool(1,pools,pso::DescriptorPoolCreateFlags::empty()).ok();
      DescSetLayout {
            layout: desc_set_layout,
            device,
            pool: pool
      }
    }
  }
  pub fn raw_layout(&self) -> &B::DescriptorSetLayout {
    self.layout.as_ref().unwrap()
  }
  pub fn create_desc_set(&mut self) -> B::DescriptorSet {
    unsafe { self.pool.as_mut().unwrap().allocate_set(self.layout.as_ref().unwrap()).unwrap() }
  }
}

impl<B: Backend> Drop for DescSetLayout<B> {
    fn drop(&mut self) {
        unsafe {
          self.device.borrow().destroy_descriptor_set_layout(self.layout.take().unwrap());
          self.device.borrow().destroy_descriptor_pool(self.pool.take().unwrap());
        }
    }
}

pub struct Uniform<B: Backend> {
    pub buffer: Option<BufferState<B>>,
    desc_set: Option<B::DescriptorSet>,
}

impl<B: Backend> Uniform<B> {
  pub fn new<T>(device:&Rc<RefCell<B::Device>>,
                memory_types: &[MemoryType],
                data: &[T],
                mut desc_set: B::DescriptorSet,
                binding:u32) -> Self where T : Copy {
    unsafe {
        let buffer = BufferState::new(
            Rc::clone(device),
            &data,
            buffer::Usage::UNIFORM,
            memory_types,
        );
        let buffer = Some(buffer);
        device.borrow_mut().write_descriptor_sets(vec![
          pso::DescriptorSetWrite { 
            set: &desc_set,
            binding: binding,
            array_offset: 0,
            descriptors: Some(pso::Descriptor::Buffer(buffer.as_ref().unwrap().get_buffer(),None .. None))
          }
        ]);
        Uniform {buffer:buffer , desc_set:Some(desc_set) }
    }
  }

  pub fn raw_desc_set(&self) -> &B::DescriptorSet {
    self.desc_set.as_ref().unwrap()
  }
}