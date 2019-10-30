extern crate gfx_backend_gl as back;
use gfx_hal as hal;
use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal::pso::{DescriptorPool};

pub struct Pipeline<B:hal::Backend> {
   pub device:Rc<RefCell<B::Device>>,
   pub desc_pool:B::DescriptorPool,
   pub desc_set_layout:B::DescriptorSetLayout,
   pub raw_pipeline:B::GraphicsPipeline,
}

impl<B> Pipeline<B> where B: hal::Backend {
  pub fn create_desc_set(&mut self) -> B::DescriptorSet {
    unsafe { self.desc_pool.allocate_set(&self.desc_set_layout) }.unwrap()
  }
}

impl<B> Drop for Pipeline<B> where B: hal::Backend {
   fn drop(&mut self) {
     
   }
}