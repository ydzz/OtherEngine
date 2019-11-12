extern crate gfx_backend_gl as back;
extern crate alga;
extern crate nalgebra as na;
use crate::graphics::shader_store::{ShaderStore};
use crate::graphics::mesh_store::{MeshStore};
use crate::graphics::render_pass::{RenderPass};
use crate::graphics::render_node::{RenderNode};
use crate::graphics::render_queue::{QueueType,RenderQueue};
use crate::graphics::gfx_helper::{DescSetLayout,Uniform};
use crate::graphics::camera::{Orthographic};
use crate::graphics::camera::{Camera};
use std::rc::{Rc};
use std::{iter};
use std::cell::{Ref,RefCell};
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
    queue,queue::family as qf
};
use gfx_hal::{DescriptorPool, Primitive, SwapchainConfig};
use gfx_hal::{Device,Adapter,adapter::{MemoryType},Instance, PhysicalDevice, Surface, Swapchain};


pub struct Graphics<B:gfx_hal::Backend> {
  surface:B::Surface,
  pub adapter:Adapter<B>,
  pub memory_types:Vec<MemoryType>,
  pub device:Rc<RefCell<B::Device>>,
  swap_chain:B::Swapchain,
  format:f::Format,
  framebuffers:Option<Vec<B::Framebuffer>>,
  frameimages:Option<Vec<(B::Image,B::ImageView)>>,

  pub mesh_store:MeshStore<B>,
  pub shader_store:Rc<ShaderStore<B>>,
  default_pass:Rc<RenderPass<B>>,
  pub command_pool:pool::CommandPool<B,queue::capability::Graphics>,
  command_buffer:RefCell< command::CommandBuffer<B,queue::capability::Graphics,command::MultiShot> >,
  viewport : pso::Viewport,
  pub queue_group :RefCell< qf::QueueGroup<B,queue::capability::Graphics>>,

  submission_complete_semaphores:B::Semaphore,

  geometry_queue   :RenderQueue<B>,
  transparent_queue:RenderQueue<B>,

  worldmvp_layout : RefCell<DescSetLayout<B>>,
  wolrdmvp_uniform:RefCell<Uniform<B>>
}

impl<B> Graphics<B> where B: gfx_hal::Backend {

  pub fn new(mut surface:B::Surface,mut adapter:Adapter<B>,winsize:Extent2D) -> Self {
    //let start = chrono::Local::now();
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let (device, queue_group) = adapter
                                        .open_with::<_, gfx_hal::Graphics>(1, |family| surface.supports_queue_family(family))
                                        .unwrap();
    let mut command_pool = unsafe {device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty())}.expect("Can't create command pool");
    println!("Memory types: {:?}", memory_types);
    let rc_device = Rc::new(RefCell::new(device));
    

    
    let mut worldmvp_layout = Graphics::create_mvp_layout(&rc_device);
    let mat:na::Matrix4<f32> = na::Matrix4::identity();
    let wolrdmvp_uniform = Uniform::new(&rc_device,&memory_types,
                                        mat.as_slice(),
                                        worldmvp_layout.create_desc_set(), 0);


    let mesh_store =  MeshStore::new(Rc::clone(&rc_device),&memory_types);

    let (swapchain,format,images,extent) = create_swapchain(winsize,&mut surface,&mut adapter,&rc_device,None);
    let render_pass = RenderPass::new_default_pass(format, Rc::clone(&rc_device));
    let ref_render_pass = Rc::new(render_pass);
    let mut shader_store = ShaderStore::new(Rc::clone(&rc_device),Rc::clone(&ref_render_pass));
    shader_store.init_builtin_shader(&worldmvp_layout);
    
    let fbos:Vec<B::Framebuffer> = images.iter().map(|&(_, ref rtv)| unsafe {
                        rc_device.borrow().create_framebuffer(ref_render_pass.get_raw_pass(), Some(rtv), extent).unwrap()
                    }).collect();

    let command_buffer = command_pool.acquire_command_buffer::<command::MultiShot>();
    let viewport = pso::Viewport {
                     rect: pso::Rect { x: 0, y: 0, w: extent.width as _, h: extent.height as _,},
                     depth: 0.0 .. 1.0
                   };
    let submission_complete_semaphores = rc_device.borrow().create_semaphore().expect("Could not create semaphore");
    //let end = chrono::Local::now();
    //println!("new graphics {} ms",end.timestamp_millis() - start.timestamp_millis());
    
    Graphics {
               surface : surface,
               adapter : adapter,
               memory_types : memory_types,
               mesh_store : mesh_store,
               shader_store : Rc::new(shader_store),
               default_pass : ref_render_pass,
               device:rc_device,
               swap_chain : swapchain,
               format: format,
               framebuffers : Some(fbos),
               frameimages : Some(images),
               command_pool : command_pool,
               command_buffer : RefCell::new(command_buffer),
               viewport : viewport,
               queue_group : RefCell::new(queue_group),
               submission_complete_semaphores : submission_complete_semaphores,
               transparent_queue:RenderQueue::new(),
               geometry_queue:RenderQueue::new(),
               worldmvp_layout : RefCell::new(worldmvp_layout),
               wolrdmvp_uniform : RefCell::new(wolrdmvp_uniform)
              }
  }

  fn create_mvp_layout(device:&Rc<RefCell<B::Device>>) -> DescSetLayout<B> {
    DescSetLayout::new(Rc::clone(device),vec![pso::DescriptorSetLayoutBinding {
                binding: 0,
                ty: pso::DescriptorType::UniformBuffer,
                count: 1,
                stage_flags: pso::ShaderStageFlags::VERTEX,
                immutable_samplers: false,
            }] )
  }

  pub fn draw(&mut self,cameras:&Vec<Camera>,lst_node:&Vec<&RenderNode<B>>) {
    let start = chrono::Local::now();
    self.geometry_queue.clear();
    self.transparent_queue.clear();
    
    let (image_idx,_) = unsafe { self.swap_chain.acquire_image(!0,None,None).unwrap() };
    self.pick_nodes_to_queue(lst_node);
    for cam in cameras {
      self.draw_queue(cam,&self.geometry_queue);
      self.draw_queue(cam,&self.transparent_queue);
    }
    unsafe { self.swap_chain.present(&mut self.queue_group.borrow_mut().queues[0],image_idx as gfx_hal::SwapImageIndex,Some(&self.submission_complete_semaphores)).unwrap(); }
    let end = chrono::Local::now();
    //println!("draw {} ms",end.timestamp_millis() - start.timestamp_millis());
  }

  pub fn pick_nodes_to_queue(&mut self,nodes:&Vec<&RenderNode<B>>) {
     for i in 0..nodes.len()  {
       let cur_node = nodes[i];
       match cur_node.material.get_shader().queue_type {
         QueueType::Geometry => self.geometry_queue.push_node(&cur_node),
         QueueType::Transparent => self.transparent_queue.push_node(&cur_node)
       }
     }
  }

  pub fn draw_queue(&self,cam:&Camera,queue:&RenderQueue<B>) {
    let framebuffers  = self.framebuffers.as_ref().unwrap();
    for shader in queue.shaders.borrow().iter() {
       let pipeline = &shader.pipelines[0];
       for material in queue.meterials.borrow().iter() {
         if material.get_shader().id != shader.id { continue }
         for i in 0..queue.meshes.borrow().len() {
           let owend_mat_id = queue.get_mesh_material_by_idx(i);
           if owend_mat_id != material.id { continue }
           unsafe {
             let mesh = &queue.meshes.borrow()[i];
             let mvp_mat4 =  cam.p_matrix() * cam.view_matrix() * &queue.mesh_mat4.borrow()[i];
             self.wolrdmvp_uniform.borrow_mut().buffer.as_mut().unwrap().update(0,mvp_mat4.as_slice());
             self.command_buffer.borrow_mut().begin(false);
             self.command_buffer.borrow_mut().set_viewports(0, &[self.viewport.clone()]);
             self.command_buffer.borrow_mut().set_scissors(0, &[self.viewport.rect]);
             self.command_buffer.borrow_mut()
                                .bind_graphics_descriptor_sets(
                                  &pipeline.pipeline_layout,
                                  0,
                                  vec![self.wolrdmvp_uniform.borrow().raw_desc_set(),material.get_desc_set()],
                                  &[]);
             self.command_buffer.borrow_mut().bind_graphics_pipeline(&pipeline.raw_pipeline);
             self.command_buffer.borrow_mut().bind_vertex_buffers(0, Some((mesh.get_raw_buffer(), 0)));
             {
               self.command_buffer.borrow_mut().begin_render_pass_inline(
                    self.default_pass.get_raw_pass(),
                    &framebuffers[0],
                    self.viewport.rect,
                    &[command::ClearValue::Color(command::ClearColor::Sfloat([
                        0.8, 0.8, 0.8, 1.0,
                    ]))],
                ).draw(0 .. mesh.vertex_count, 0 .. 3);
             }
             self.command_buffer.borrow_mut().finish();
             let ques = &mut self.queue_group.borrow_mut().queues;
             Ref::map(self.command_buffer.borrow(), |mi| { 
               ques[0].submit_without_semaphores(Some(mi),None);
               mi
             });
           }
         }
       }
    }
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

  pub fn get_winsize(&self) -> (u32,u32)   {
    (self.viewport.rect.w as u32,self.viewport.rect.h as u32)
  }
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

pub const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0 .. 1,
    layers: 0 .. 1,
};
