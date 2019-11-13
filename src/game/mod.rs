pub mod pool;
pub mod image;
mod view_node;
use crate::graphics::camera::Camera;
use crate::graphics::Graphics;
use gfx_hal::window::Extent2D;
use std::cell::RefCell;
pub struct Game {
    graphics: RefCell<Graphics>,
    camera_list: Vec<Camera>,

}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        Game {
            graphics: RefCell::new(graphics),
            camera_list: Vec::new()
        }
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.camera_list.push(camera);
    }

    pub fn update(&self) {
        self.graphics.borrow_mut().draw(&self.camera_list, &vec![]);
    }

    pub fn resize_view(&self, size: Extent2D) {
        self.graphics.borrow_mut().recreate_swapchain(size);
    }
}
