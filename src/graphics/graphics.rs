extern crate alga;
extern crate nalgebra as na;
use crate::graphics::camera::Camera;
use crate::graphics::gfx_helper::{DescSetLayout,Swapchain as GfxSwapchain,FrameBuffer,Uniform,GPBackend};
use crate::graphics::mesh_store::MeshStore;
use crate::graphics::render_node::RenderNode;
use crate::graphics::render_pass::RenderPass;
use crate::graphics::render_queue::{QueueType, RenderQueue};
use crate::graphics::shader_store::ShaderStore;
use gfx_hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{PipelineStage, ShaderStageFlags, VertexInputRate};
use gfx_hal::queue::Submission;
use gfx_hal::{
  adapter::{Adapter, MemoryType, PhysicalDevice},
  buffer, command,
  command::CommandBuffer,
  device::Device,
  format as f, image as i, memory as m, pass, pool,
  pool::CommandPool,
  pso,
  pso::{DescriptorPool, Primitive},
  queue,
  queue::family as qf,
  queue::{family::QueueFamily, CommandQueue},
  window::Extent2D,
  window::{Surface, SwapchainConfig,Swapchain},
};
use std::cell::{Ref, RefCell};
use std::iter;
use std::rc::Rc;

pub struct Graphics<B: gfx_hal::Backend> {
  pub memory_types: Vec<MemoryType>,
  pub device: Rc<RefCell<B::Device>>,
  swapchain: Option<GfxSwapchain<B>>,
  framebuffer: FrameBuffer<B>,
  pub mesh_store: MeshStore<B>,
  pub shader_store: Rc<RefCell< ShaderStore<B>>>,
  default_pass: Rc<RenderPass<B>>,
  pub command_pool: B::CommandPool,
  command_buffer: RefCell<B::CommandBuffer>,
  pub viewport: pso::Viewport,
  pub queue_group: RefCell<qf::QueueGroup<B>>,
  geometry_queue: RenderQueue<B>,
  transparent_queue: RenderQueue<B>,
  pub worldmvp_layout: RefCell<DescSetLayout<B>>,
  backend:GPBackend<B>

}

impl<B> Graphics<B>
where
  B: gfx_hal::Backend,
{
  pub fn new(mut gp_backend: GPBackend<B>, winsize: Extent2D) -> Self {
    //let mut rc_backend = Rc::new(gp_backend);
    let memory_types: Vec<MemoryType> = gp_backend.rc_adapter().physical_device.memory_properties().memory_types;

    let family = gp_backend.rc_adapter().queue_families.iter().find(|family| {
      gp_backend.rc_surface().supports_queue_family(family) && family.queue_type().supports_graphics()
    }).unwrap();

    let mut gpu = unsafe {
      gp_backend.rc_adapter().physical_device.open(&[(family, &[1.0])], gfx_hal::Features::empty()).unwrap()
    };
    let queues = gpu.queue_groups.pop().unwrap();
    let mut command_pool = unsafe {
      gpu.device.create_command_pool(queues.family, pool::CommandPoolCreateFlags::empty()).expect("Can't create command pool")
    };
    
    let rc_device = Rc::new(RefCell::new(gpu.device));

    let worldmvp_layout = Graphics::create_mvp_layout(&rc_device);

    let mut swapchain = GfxSwapchain::new(&mut gp_backend, rc_device.clone(),winsize);

    let render_pass = RenderPass::new_default_pass(swapchain.format, Rc::clone(&rc_device));
    let ref_render_pass = Rc::new(render_pass);

    let mesh_store = MeshStore::new(Rc::clone(&rc_device), &memory_types);
    let mut shader_store = ShaderStore::new(Rc::clone(&rc_device), Rc::clone(&ref_render_pass));
    shader_store.init_builtin_shader(&worldmvp_layout);
    
    let framebuffer = FrameBuffer::new(rc_device.clone(),ref_render_pass.get_raw_pass(),&mut swapchain,&queues);

    let  command_buffer = unsafe { command_pool.allocate_one(command::Level::Primary) };
    let viewport = Graphics::create_viewport(&swapchain);

    Graphics {
      memory_types: memory_types,
      mesh_store: mesh_store,
      shader_store: Rc::new(RefCell::new(shader_store)),
      default_pass: ref_render_pass,
      device: rc_device,
      swapchain: Some(swapchain),
      framebuffer: framebuffer,
      command_pool: command_pool,
      command_buffer: RefCell::new(command_buffer),
      viewport: viewport,
      queue_group: RefCell::new(queues),
      transparent_queue: RenderQueue::new(),
      geometry_queue: RenderQueue::new(),
      worldmvp_layout: RefCell::new(worldmvp_layout),
      backend: gp_backend
    }
  }

  fn create_mvp_layout(device: &Rc<RefCell<B::Device>>) -> DescSetLayout<B> {
    DescSetLayout::new(
      Rc::clone(device),
      vec![pso::DescriptorSetLayoutBinding {
        binding: 0,
        ty: pso::DescriptorType::UniformBuffer,
        count: 1,
        stage_flags: pso::ShaderStageFlags::VERTEX,
        immutable_samplers: false,
      }],1000
    )
  }

  fn create_viewport(swapchain: &GfxSwapchain<B>) -> pso::Viewport {
    pso::Viewport {
            rect: pso::Rect {
                x: 0,
                y: 0,
                w: swapchain.extent.width as i16,
                h: swapchain.extent.height as i16,
            },
            depth: 0.0 .. 1.0
    }
  }

  pub fn draw(&mut self, cameras: &Vec<Camera>, lst_node: &Vec<Rc<RenderNode<B>>>) {
    self.geometry_queue.clear();
    self.transparent_queue.clear();
    //let (image_idx, _) = unsafe { self.swap_chain.acquire_image(!0, None, None).unwrap() };
    self.pick_nodes_to_queue(lst_node);
    for cam in cameras {
      unsafe {
        self.draw_queue(cam);
        //self.draw_queue(cam, &self.transparent_queue);
      }
    }
    unsafe {
      //self.swap_chain.present(&mut self.queue_group.borrow_mut().queues[0],image_idx,Some(&self.submission_complete_semaphores)).unwrap();
    }
  }

  pub fn pick_nodes_to_queue(&mut self, nodes: &Vec<Rc<RenderNode<B>>>) {
    for i in 0..nodes.len() {
      let cur_node = &nodes[i];
      match cur_node.material.get_shader().queue_type {
        QueueType::Geometry => self.geometry_queue.push_node(&cur_node),
        QueueType::Transparent => self.transparent_queue.push_node(&cur_node),
      }
    }
  }

  pub unsafe fn draw_queue(&mut self, cam: &Camera) {
    let queue = &self.geometry_queue;
    let sem_index = self.framebuffer.next_acq_pre_pair_index();
    let (acquire_semaphore, _) = self.framebuffer.get_frame_data(None, Some(sem_index)).1.unwrap();
    
    let acquire_image = self.swapchain.as_mut().unwrap().swapchain.as_mut().unwrap().acquire_image(!0, Some(acquire_semaphore), None);
    let frame = match acquire_image {
      Ok((i, _)) => i,
      Err(_) => { return; }
    };

    
    let (fid, sid) = self.framebuffer.get_frame_data(Some(frame as usize), Some(sem_index));
    let (framebuffer_fence, framebuffer, command_pool, command_buffers) = fid.unwrap();
    let (image_acquired, image_present) = sid.unwrap();

    self.device.borrow().wait_for_fence(framebuffer_fence, !0).unwrap();
    self.device.borrow().reset_fence(framebuffer_fence).unwrap();
    command_pool.reset(false);
    
    let mut cmd_buffer = match command_buffers.pop() {
      Some(cmd_buffer) => cmd_buffer,
      None => command_pool.allocate_one(command::Level::Primary),
    };
    cmd_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);
    cmd_buffer.set_viewports(0, &[self.viewport.clone()]);
    cmd_buffer.set_scissors(0, &[self.viewport.rect]);
    cmd_buffer.begin_render_pass(
      self.default_pass.get_raw_pass(),
      framebuffer,
      self.viewport.rect,
      &[command::ClearValue { color: command::ClearColor { float32: [0.8, 0.8, 0.8, 1.0] }}],command::SubpassContents::Inline);
    let vp_mat4 =  cam.p_matrix() * cam.view_matrix();
    for shader in queue.shaders.borrow().iter() {
      let pipeline = &shader.pipelines[0];
      cmd_buffer.bind_graphics_pipeline(&pipeline.raw_pipeline);
      
      for material in queue.meterials.borrow().iter() {
        if material.get_shader().id != shader.id {
          continue;
        }
        for i in 0..queue.meshes.borrow().len() {
          let owend_mat_id = queue.get_mesh_material_by_idx(i);
          if owend_mat_id != material.id {
            continue;
          }
          //update mvp matrix
          let mesh = &queue.meshes.borrow()[i];
          let uniform = &queue.mesh_uniform.borrow()[i];
          let mat = &queue.mesh_mat4.borrow()[i];
          let mvp_mat4 =  vp_mat4 * mat;
         
          uniform.borrow_mut().buffer.as_mut().unwrap().update(0,mvp_mat4.as_slice());

          cmd_buffer.bind_graphics_descriptor_sets(&pipeline.pipeline_layout,0,vec![uniform.borrow().raw_desc_set(),material.get_desc_set(),],&[]);
          cmd_buffer.bind_vertex_buffers(0, Some((mesh.get_raw_buffer(), 0)));
  
          cmd_buffer.draw(0..mesh.vertex_count, 0..1);        
        }
      }
    }
    cmd_buffer.end_render_pass();
    cmd_buffer.finish();
    let submission = Submission {
      command_buffers: iter::once(&cmd_buffer),
      wait_semaphores: iter::once((&*image_acquired, pso::PipelineStage::BOTTOM_OF_PIPE)),
      signal_semaphores: iter::once(&*image_present),
    };
    self.queue_group.borrow_mut().queues[0].submit(submission, Some(framebuffer_fence));
    command_buffers.push(cmd_buffer);
    self.swapchain.as_mut().unwrap().swapchain.as_mut().unwrap().present(
                    &mut self.queue_group.borrow_mut().queues[0],
                    frame,
                    Some(&*image_present)).unwrap();
  }

  pub fn recreate_swapchain(&mut self, newsize: Extent2D) {
    self.device.borrow().wait_idle().unwrap();
    self.swapchain.take().unwrap();
    self.swapchain = Some(GfxSwapchain::new(&mut self.backend, Rc::clone(&self.device),newsize) );
    self.default_pass = Rc::new(RenderPass::new_default_pass(self.swapchain.as_ref().unwrap().format,self.device.clone()));
    let buffer = FrameBuffer::new(self.device.clone(), self.default_pass.get_raw_pass(), self.swapchain.as_mut().unwrap(), &self.queue_group.borrow());
    self.framebuffer = buffer;
    self.viewport = Graphics::create_viewport(self.swapchain.as_ref().unwrap());
    self.shader_store.borrow_mut().re_create_render_pass(self.default_pass.clone(), &self.worldmvp_layout.borrow());
  }

}