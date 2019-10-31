use crate::graphics::gfx_helper::{BufferState};
pub struct Mesh<B:gfx_hal::Backend> {
  pub buffer:BufferState<B>
}

impl<B> Mesh<B> where B:gfx_hal::Backend {
  pub fn get_raw_buffer(&self)-> &B::Buffer  {
    self.buffer.get_buffer()
  }
}