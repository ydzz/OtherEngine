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
}

impl Graphics {
  pub fn new(surface:back::Surface,adapter:Adapter<back::Backend>) -> Self {
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let (device, mut queue_group) = adapter
                                        .open_with::<_, gfx_hal::Graphics>(1, |family| surface.supports_queue_family(family))
                                        .unwrap();

    let mut command_pool = unsafe {
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty())
    }.expect("Can't create command pool");
  
    println!("Memory types: {:?}", memory_types);
    Graphics {surface : surface, adapter : adapter,device:device}
  }

 
}