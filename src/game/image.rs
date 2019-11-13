use crate::game::pool::Pool;
use crate::graphics::RenderNode;
use gfx_hal::Backend;
pub struct Image {
    width: f32,
    height: f32,
    node: Option<RenderNode>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 100f32,
            height: 100f32,
            node: None,
        }
    }
    pub fn create_pool() -> Pool<Image> {
        Pool::create(|| Image::new(), |t| {})
    }

    pub fn useImage() {}
}
