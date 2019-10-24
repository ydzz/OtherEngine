extern crate gfx_backend_gl as back;
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

pub struct Graphics {
  surface:back::Surface,
  adapter:Adapter<back::Backend>,

  device:back::Device,
  swap_chain:back::Swapchain,
  format:f::Format
}

pub fn create_swapchain(winsize:Extent2D,mut surface:&mut back::Surface,
                        adapter:&mut Adapter<back::Backend>,device:&mut back::Device,may_format:Option<f::Format>)
    -> (back::Swapchain,f::Format)       {
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
  let (swapchain,_) = unsafe { device.create_swapchain(&mut surface, swap_config, None) }.expect("Can't create swapchain");
  (swapchain, format.unwrap())
}

impl Graphics {
  pub fn new(mut surface:back::Surface,mut adapter:Adapter<back::Backend>,winsize:Extent2D) -> Self {
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let (mut device, mut queue_group) = adapter
                                        .open_with::<_, gfx_hal::Graphics>(1, |family| surface.supports_queue_family(family))
                                        .unwrap();
    let mut command_pool = unsafe {
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty())
    }.expect("Can't create command pool");
    println!("Memory types: {:?}", memory_types);

    let (swapchain,format) = create_swapchain(winsize,&mut surface,&mut adapter,&mut device,None);
     Graphics {surface : surface, adapter : adapter,device:device, swap_chain:swapchain,format: format}
  }

  pub fn draw(&mut self) {
    //self.swap_chain.acquire_image(_timeout_ns: u64, _semaphore: Option<&native::Semaphore>, _fence: Option<&native::Fence>)
  }

  pub fn recreate_swapchain(&mut self,newsize:Extent2D) {
    self.device.wait_idle().unwrap();
    let (swapchain,_) = create_swapchain(newsize,&mut self.surface,&mut self.adapter,&mut self.device,Some(self.format));
    self.swap_chain = swapchain;
  }
}