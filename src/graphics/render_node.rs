use crate::graphics::mesh::{Mesh};
use crate::graphics::graphics::{Graphics};
use crate::graphics::material::{Material};
use std::rc::{Rc};
use std::cell::{RefCell};
use crate::graphics::gfx_helper::{Uniform};
use crate::graphics::transform::{Transform};
use nalgebra::{Matrix4};
pub struct RenderNode<B:gfx_hal::Backend> {
 pub mat4:Matrix4<f32>,
 pub mesh:Rc<Mesh<B>>,
 pub material:Rc<Material<B>>,

 pub uniform:Rc<RefCell<Uniform<B>>>,
}

impl<B> RenderNode<B> where B:gfx_hal::Backend {
  pub fn new(gp:&RefCell<Graphics<B>>,transform:&Transform,mesh:&Rc<Mesh<B>>,material:&Rc<Material<B>>) -> Self {
      let mat = transform.matrix();
      let desc_set = gp.borrow().worldmvp_layout.borrow_mut().create_desc_set();
      let uniform = Uniform::new(&gp.borrow().device,&gp.borrow().memory_types,mat.as_slice(),desc_set,0);
      RenderNode {
          mat4 : transform.matrix(),
          mesh : Rc::clone(mesh),
          material : Rc::clone(material),
          uniform : Rc::new(RefCell::new(uniform))
      }
  }
}