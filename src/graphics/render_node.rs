use crate::graphics::mesh::{Mesh};
use crate::graphics::material::{Material};
use nalgebra::{Matrix4};
use std::rc::{Rc};
pub struct RenderNode<B:gfx_hal::Backend> {
 pub mat:Matrix4<f32>,
 pub mesh:Rc<Mesh<B>>,
 pub material:Rc<Material<B>>
}