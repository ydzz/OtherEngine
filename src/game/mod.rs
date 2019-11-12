use crate::graphics::{Graphics};
use crate::graphics::camera::{Camera};
use std::cell::{RefCell};
use gfx_hal::window::{Extent2D};
pub struct Game<B:gfx_hal::Backend> {
    graphics:RefCell<Graphics<B>>,
    camera_list:Vec<Camera>
}

impl<B> Game<B> where B:gfx_hal::Backend {
    pub fn new(graphics:Graphics<B>) -> Self {
     Game { graphics : RefCell::new(graphics), camera_list : Vec::new() }
    }

    pub fn add_camera(&mut self,camera:Camera) {
      self.camera_list.push(camera);
    }

    pub fn update(&self) {
        self.graphics.borrow_mut().draw(&self.camera_list, &vec![]);
    }

    pub fn resize_view(&self,size:Extent2D) {
        self.graphics.borrow_mut().recreate_swapchain(size);
    }
} 