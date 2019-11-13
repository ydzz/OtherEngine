use crate::graphics::mesh::{Mesh};
use crate::graphics::material::{Material};
use std::rc::{Rc};
use nalgebra::{Matrix4};
pub struct RenderNode<B:gfx_hal::Backend> {
 pub mat4:Matrix4<f32>,
 pub mesh:Rc<Mesh<B>>,
 pub material:Rc<Material<B>>
}