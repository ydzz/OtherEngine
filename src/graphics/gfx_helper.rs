use std::mem::{size_of, ManuallyDrop};
use std::{iter, ptr};
use image::GenericImageView;
use gfx_hal::{ Backend,
               device::{Device}, 
               buffer, pool,queue::family::{QueueGroup},
               adapter::{Adapter, MemoryType},
               memory as m,
               pso,window::{Surface},window as w,
               format::{self as f, AsFormat},
               image as i
             };
use gfx_hal::pso::DescriptorPool;
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
    
    let mapping = device.borrow().map_memory(&image_upload_memory, 0 .. image_mem_reqs.size).unwrap();
    let img = &dynimage.to_rgba();
    println!("{:?},len:{}",img.dimensions(),img.len());
    for y in 0 .. height as usize {
      let data_source_slice = &(**img)
          [y * (width as usize) * image_stride .. (y + 1) * (width as usize) * image_stride];
      ptr::copy_nonoverlapping(data_source_slice.as_ptr(),mapping.offset(y as isize * row_pitch as isize),data_source_slice.len());
    }
    device.borrow_mut().unmap_memory(&image_upload_memory);

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
  pub  fn new(device:Rc<RefCell<B::Device>>, bindings: Vec<pso::DescriptorSetLayoutBinding>,size:usize) -> Self {
    unsafe {
      let pools:Vec<pso::DescriptorRangeDesc> = bindings.iter().map(|binding| pso::DescriptorRangeDesc {
        ty : binding.ty,
        count : binding.count
      }).collect();
      let desc_set_layout = device.borrow().create_descriptor_set_layout(bindings, &[]).ok();
      let pool = device.borrow().create_descriptor_pool(size,pools,pso::DescriptorPoolCreateFlags::empty()).ok();
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
    unsafe { self.pool.as_mut().unwrap().allocate_set(self.layout.as_ref().unwrap()).expect("?????????????") }
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

pub struct GPBackend <B:Backend> {
  pub surface:B::Surface,
  pub adapter:Adapter<B>
}

impl<B> GPBackend<B> where B:Backend {
  pub fn rc_surface(&self) -> &B::Surface {
    &self.surface
  }
  pub fn rc_adapter(&self) -> &Adapter<B> {
    &self.adapter
  }
}



pub struct FrameBuffer<B:Backend> {
  framebuffers: Option<Vec<B::Framebuffer>>,
  framebuffer_fences: Option<Vec<B::Fence>>,
  command_pools: Option<Vec<B::CommandPool>>,
  command_buffer_lists: Vec<Vec<B::CommandBuffer>>,
  frame_images: Option<Vec<(B::Image, B::ImageView)>>,
  acquire_semaphores: Option<Vec<B::Semaphore>>,
  present_semaphores: Option<Vec<B::Semaphore>>,
  last_ref: usize,
  device: Rc<RefCell<B::Device>>,
}

impl<B> FrameBuffer<B> where B:Backend {
  pub fn new(device: Rc<RefCell<B::Device>>,render_pass: &B::RenderPass,swapchain: &mut Swapchain<B>,queues:&QueueGroup<B>) -> Self {
    let extent = i::Extent { 
                             width: swapchain.extent.width as _,
                             height: swapchain.extent.height as _,
                             depth: 1 
                           };
    let frame_images = swapchain.backbuffer.take().unwrap().into_iter()
                                    .map(|image| {
                                         let rtv = unsafe { device.borrow().create_image_view(
                                                             &image,i::ViewKind::D2,
                                                             swapchain.format,
                                                             f::Swizzle::NO,COLOR_RANGE.clone()).unwrap() };
                                         (image,rtv)}).collect::<Vec<_>>();
    let framebuffers = frame_images.iter().map(|&(_, ref rtv)| {
      unsafe { device.borrow().create_framebuffer(render_pass,Some(rtv),extent).unwrap() }
    }).collect();

    let iter_count = if frame_images.len() != 0 { frame_images.len() } else { 1  };

    let mut fences: Vec<B::Fence> = vec![];
    let mut command_pools: Vec<_> = vec![];
    let mut command_buffer_lists = Vec::new();
    let mut acquire_semaphores: Vec<B::Semaphore> = vec![];
    let mut present_semaphores: Vec<B::Semaphore> = vec![];

    for _ in 0 .. iter_count {
      fences.push(device.borrow().create_fence(true).unwrap());
      unsafe {
        command_pools.push(
          device.borrow().create_command_pool(queues.family,pool::CommandPoolCreateFlags::empty()).expect("Can't create command pool"),
        );
      }
      command_buffer_lists.push(Vec::new());
      acquire_semaphores.push(device.borrow().create_semaphore().unwrap());
      present_semaphores.push(device.borrow().create_semaphore().unwrap());
    }
    FrameBuffer {
      frame_images: Some(frame_images),
      framebuffers: Some(framebuffers),
      framebuffer_fences: Some(fences),
      command_pools: Some(command_pools),
      command_buffer_lists,
      present_semaphores: Some(present_semaphores),
      acquire_semaphores: Some(acquire_semaphores),
      device,
      last_ref: 0,
    }
  }

  pub fn next_acq_pre_pair_index(&mut self) -> usize {
    if self.last_ref >= self.acquire_semaphores.as_ref().unwrap().len() {
        self.last_ref = 0
    }

    let ret = self.last_ref;
    self.last_ref += 1;
    ret
  }

  pub fn get_frame_data(&mut self,frame_id: Option<usize>,sem_index: Option<usize>) -> (
    Option<(&mut B::Fence,&mut B::Framebuffer,&mut B::CommandPool,&mut Vec<B::CommandBuffer>)>,
    Option<(&mut B::Semaphore, &mut B::Semaphore)>) {
      (
        if let Some(fid) = frame_id {
            Some((
                &mut self.framebuffer_fences.as_mut().unwrap()[fid],
                &mut self.framebuffers.as_mut().unwrap()[fid],
                &mut self.command_pools.as_mut().unwrap()[fid],
                &mut self.command_buffer_lists[fid],
            ))
        } else {
            None
        },
        if let Some(sid) = sem_index {
            Some((
                &mut self.acquire_semaphores.as_mut().unwrap()[sid],
                &mut self.present_semaphores.as_mut().unwrap()[sid],
            ))
        } else {
            None
        },
    ) 
  }
}

pub struct Swapchain<B:Backend> {
  pub swapchain: Option<B::Swapchain>,
  backbuffer: Option<Vec<B::Image>>,
  device: Rc<RefCell<B::Device>>,
  pub extent: i::Extent,
  pub format: f::Format,
}

impl<B> Swapchain<B> where B:Backend {
  pub fn new(backend:&mut GPBackend<B>,device: Rc<RefCell<B::Device>>,wsize:w::Extent2D) -> Self {
    let caps = backend.surface.capabilities(&backend.adapter.physical_device);
    let formats = backend.surface.supported_formats(&backend.adapter.physical_device);
    let format = formats.map_or(f::Format::Rgba8Srgb, |formats| {
      formats.iter()
             .find(|format| format.base_format().1 == f::ChannelType::Srgb)
             .map(|format| *format).unwrap_or(formats[0]) });
    let swap_config = w::SwapchainConfig::from_caps(&caps, format, wsize);
    let extent = swap_config.extent.to_extent();
    let (swapchain, backbuffer) = unsafe { device.borrow()
                                                 .create_swapchain(&mut backend.surface, swap_config, None)
                                                 .expect("Can't create swapchain")
                                  };
    Swapchain {
      swapchain : Some(swapchain),
      backbuffer : Some(backbuffer),
      device,
      extent,
      format,
    }
  }
}

pub const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
  aspects: f::Aspects::COLOR,
  levels: 0..1,
  layers: 0..1,
};