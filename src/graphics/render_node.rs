use crate::graphics::mesh::{Mesh};
use crate::graphics::material::{Material};
use std::rc::{Rc};
pub struct RenderNode<B:gfx_hal::Backend> {
 pub mesh:Rc<Mesh<B>>,
 pub material:Rc<Material<B>>
}