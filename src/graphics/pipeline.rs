extern crate gfx_backend_gl as back;
use gfx_hal as hal;
use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal::pso::{DescriptorPool};
use crate::graphics::gfx_helper::{DescSetLayout};

pub struct Pipeline<B:hal::Backend> {
   pub device:Rc<RefCell<B::Device>>,
   pub raw_pipeline:B::GraphicsPipeline,
   pub pipeline_layout:B::PipelineLayout,
   pub desc_set_layout:RefCell<DescSetLayout<B>>
}

impl<B> Pipeline<B> where B: hal::Backend {
  pub fn create_desc_set(&self) -> B::DescriptorSet {
    self.desc_set_layout.borrow_mut().create_desc_set()
  }
}

impl<B> Drop for Pipeline<B> where B: hal::Backend {
   fn drop(&mut self) {
     
   }
}