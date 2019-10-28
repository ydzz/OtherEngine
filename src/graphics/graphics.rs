extern crate gfx_backend_gl as back;
use crate::graphics::pipeline::{Pipeline};
use crate::graphics::mesh_store::{MeshStore};
use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{PipelineStage, ShaderStageFlags, VertexInputRate};
use gfx_hal::queue::Submission;
use gfx_hal::{
    buffer,
    command,
    format as f,
    image as i,
    memory as m,
    pass,
    pool,
    pso,
    window::Extent2D,
};
use gfx_hal::{DescriptorPool, Primitive, SwapchainConfig};
use gfx_hal::{Device, Adapter,Instance, PhysicalDevice, Surface, Swapchain};


pub struct Graphics<B:gfx_hal::Backend> {
  surface:B::Surface,
  adapter:Adapter<B>,

  device:Rc<RefCell<B::Device>>,
  swap_chain:B::Swapchain,
  format:f::Format,

  mesh_store:MeshStore<B>
}

pub fn create_swapchain<B:gfx_hal::Backend>(winsize:Extent2D,mut surface:&mut B::Surface,
                        adapter:&mut Adapter<B>,device:Rc<RefCell<B::Device>>,may_format:Option<f::Format>)
    -> (B::Swapchain,f::Format)       {
   let (caps, formats, _present_modes) = surface.compatibility(&mut adapter.physical_device);
   let format = may_format.or_else(|| {
       let format =  formats.map_or(f::Format::Rgba8Srgb, |formats| {
        formats.iter().find(|format| format.base_format().1 == ChannelType::Srgb)
                      .map(|format| *format)
                      .unwrap_or(formats[0])
    });
    Some(format)
   });
   let swap_config = SwapchainConfig::from_caps(&caps, format.unwrap(), winsize);
   
   //let extent = swap_config.extent.to_extent();
  let (swapchain,backbuffer) = unsafe { device.borrow().create_swapchain(&mut surface, swap_config, None) }.expect("Can't create swapchain");
  let pairs = backbuffer.into_iter().map(|image| unsafe {
              let rtv = device.borrow().create_image_view(&image, i::ViewKind::D2, format.unwrap(), Swizzle::NO, COLOR_RANGE.clone()).unwrap();
              (image,rtv)
            }).collect::<Vec<_>>();
  //let fbos = pairs.iter().map(|&(_, ref rtv)| unsafe {});
  //device.create_graphics_pipeline(desc: &pso::GraphicsPipelineDesc<'a, B>, _cache: Option<&()>)
  
  (swapchain, format.unwrap())
}

impl<B> Graphics<B> where B: gfx_hal::Backend {
  pub fn new(mut surface:B::Surface,mut adapter:Adapter<B>,winsize:Extent2D) -> Self {
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let (mut device, queue_group) = adapter
                                        .open_with::<_, gfx_hal::Graphics>(1, |family| surface.supports_queue_family(family))
                                        .unwrap();
    let mut command_pool = unsafe {device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty())}.expect("Can't create command pool");
    println!("Memory types: {:?}", memory_types);
    let rc_device = Rc::new(RefCell::new(device));
    let mesh_store =  MeshStore::new(Rc::clone(&rc_device),&memory_types);
    
    let (swapchain,format) = create_swapchain(winsize,&mut surface,&mut adapter,Rc::clone(&rc_device),None);
     Graphics {surface : surface, adapter : adapter, 
               device:rc_device, swap_chain : swapchain,format: format,
               mesh_store : mesh_store
              }
  }

  pub fn draw(&mut self) {
    //self.swap_chain.acquire_image(_timeout_ns: u64, _semaphore: Option<&native::Semaphore>, _fence: Option<&native::Fence>)
  }

  pub fn recreate_swapchain(&mut self,newsize:Extent2D) {
    self.device.borrow().wait_idle().unwrap();
    let (swapchain,_) = create_swapchain(newsize,&mut self.surface,&mut self.adapter,Rc::clone(&self.device),Some(self.format));
    self.swap_chain = swapchain;
  }
}


const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0 .. 1,
    layers: 0 .. 1,
};