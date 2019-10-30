use gfx_hal::{
              pass,
              format,
              image as i,
              device::{Device}
             };
use std::mem::{self, ManuallyDrop};
use std::rc::{Rc};
use std::cell::{RefCell};
pub struct RenderPass<B:gfx_hal::Backend> {
  raw_pass:Option<B::RenderPass>,
  device:Rc<RefCell<B::Device>>
}

impl<B> RenderPass<B> where B:gfx_hal::Backend {
  pub fn new_default_pass(format:format::Format,device:Rc<RefCell<B::Device>>) -> Self {
    let attachment = pass::Attachment {
                format: Some(format),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: i::Layout::Undefined .. i::Layout::Present,
            };
    let subpass = pass::SubpassDesc {
                colors: &[(0, i::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };
    let pass = unsafe { device.borrow().create_render_pass(&[attachment], &[subpass], &[]) }.ok();
    RenderPass {
        raw_pass : pass,
        device : device
    }
  }

  pub fn get_raw_pass(&self) -> &B::RenderPass {
      self.raw_pass.as_ref().unwrap()
  }
}

impl<B> Drop for RenderPass<B> where B:gfx_hal::Backend {
  fn drop(&mut self) {
    let device = &self.device.borrow();
    unsafe { device.destroy_render_pass(self.raw_pass.take().unwrap()); }
  }
}