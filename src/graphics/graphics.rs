extern crate gfx_backend_gl as back;
use crate::graphics::shader_store::{ShaderStore};
use crate::graphics::mesh_store::{MeshStore};
use crate::graphics::render_pass::{RenderPass};
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
  framebuffers:Option<Vec<B::Framebuffer>>,
  frameimages:Option<Vec<(B::Image,B::ImageView)>>,

  mesh_store:MeshStore<B>,
  shader_store:ShaderStore<B>,
  default_pass:Rc<RenderPass<B>>
}

pub fn create_swapchain<B:gfx_hal::Backend>(winsize:Extent2D,mut surface:&mut B::Surface,
                        adapter:&mut Adapter<B>,device:&RefCell<B::Device>,may_format:Option<f::Format>)
    -> (B::Swapchain,f::Format,Vec<(B::Image,B::ImageView)>,gfx_hal::image::Extent)       {
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
   let extent = swap_config.extent.to_extent();
  let (swapchain,backbuffer) = unsafe { device.borrow().create_swapchain(&mut surface, swap_config, None) }.expect("Can't create swapchain");
  println!("len:{}",backbuffer.len());
  let pairs = backbuffer.into_iter().map(|image| unsafe {
              let rtv = device.borrow().create_image_view(&image, i::ViewKind::D2, format.unwrap(), Swizzle::NO, COLOR_RANGE.clone()).unwrap();
              (image,rtv)
            }).collect::<Vec<_>>();
  
  
  (swapchain, format.unwrap(),pairs,extent)
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
    
    let (swapchain,format,images,extent) = create_swapchain(winsize,&mut surface,&mut adapter,&rc_device,None);
    let render_pass = RenderPass::new_default_pass(format, Rc::clone(&rc_device));
    let ref_render_pass = Rc::new(render_pass);
    let mut shader_store = ShaderStore::new(Rc::clone(&rc_device),Rc::clone(&ref_render_pass));
    shader_store.init_builtin_shader();

    let fbos:Vec<B::Framebuffer> = images.iter().map(|&(_, ref rtv)| unsafe {
                        rc_device.borrow().create_framebuffer(ref_render_pass.get_raw_pass(), Some(rtv), extent).unwrap()
                    }).collect();
    Graphics {
               surface : surface, 
               adapter : adapter,                
               mesh_store : mesh_store,
               shader_store : shader_store,
               default_pass : ref_render_pass,
               device:rc_device, 
               swap_chain : swapchain,
               format: format,
               framebuffers : Some(fbos),
               frameimages : Some(images)
              }
  }

  pub fn draw(&mut self) {
    let (image_idx,_) = unsafe { self.swap_chain.acquire_image(!0,None,None).unwrap() };
    //println!("{}",image_idx);
  }

  pub fn recreate_swapchain(&mut self,newsize:Extent2D) {
    unsafe {
      let mut frames = self.framebuffers.take().unwrap();
      let mut images = self.frameimages.take().unwrap();
      for frame in frames {
        self.device.borrow().destroy_framebuffer(frame);
      }
      for (_,imageview) in images {
        self.device.borrow().destroy_image_view(imageview);
      }
    }
    self.device.borrow().wait_idle().unwrap();
    let (swapchain, _ , images,extent) = create_swapchain(newsize,&mut self.surface,&mut self.adapter,&self.device,Some(self.format));
    
    let fbos:Vec<B::Framebuffer> = images.iter().map(|&(_, ref rtv)| unsafe {
                        self.device.borrow().create_framebuffer(self.default_pass.get_raw_pass(), Some(rtv), extent).unwrap()
                    }).collect();
    
    self.framebuffers = Some(fbos);
    self.frameimages = Some(images);
    self.swap_chain = swapchain;
  }
}


const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0 .. 1,
    layers: 0 .. 1,
};