use std::rc::{Rc};
use gfx_hal as hal;
use crate::graphics::shader::{Shader};

pub struct Material<B:hal::Backend> {
  pub shader:Rc<Shader<B>>
}