extern crate gfx_backend_gl as back;
use gfx_hal as hal;

pub struct Pipeline<B:hal::Backend> {
   vs_shader:Option<B::ShaderModule>,
   ps_shader:Option<B::ShaderModule>
}

impl<B> Pipeline<B> where B: hal::Backend {
  pub fn new() -> Self {
    Pipeline { vs_shader:None, ps_shader:None }
  }
}

impl<B> Drop for Pipeline<B> where B: hal::Backend {
   fn drop(&mut self) {
     
   }
}