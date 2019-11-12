use crate::graphics::mesh::{Mesh};
use crate::graphics::material::{Material};
use crate::graphics::transform::{Transform};
use std::rc::{Rc};
pub struct RenderNode<B:gfx_hal::Backend> {
 pub transform:Transform,
 pub mesh:Rc<Mesh<B>>,
 pub material:Rc<Material<B>>
}