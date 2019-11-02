use std::collections::{HashMap,HashSet};
use crate::graphics::shader::{Shader};
use crate::graphics::material::{Material};
use crate::graphics::mesh::{Mesh};
use crate::graphics::render_node::{RenderNode};
use std::cell::{RefCell};
use std::rc::{Rc};
pub enum QueueType {
    Geometry,
    Transparent
}

pub struct RenderQueue<B:gfx_hal::Backend> {
   shader_dic:RefCell<HashMap<u128,usize>>,
   material_dic:RefCell<HashMap<u128,usize>>,
   pub mesh_dic:RefCell<HashMap<usize,u128>>,

   pub shaders:RefCell<Vec<Rc<Shader<B>>>>,
   pub meterials:RefCell<Vec<Rc<Material<B>>>>,
   pub meshes:RefCell<Vec<Rc<Mesh<B>>>>
}

impl<B> RenderQueue<B> where B:gfx_hal::Backend {
  pub fn new() -> Self {
      RenderQueue {
        shader_dic:RefCell::new(HashMap::new()),
        material_dic:RefCell::new(HashMap::new()),
        mesh_dic:RefCell::new(HashMap::new()),
        shaders:RefCell::new(Vec::new()),
        meterials:RefCell::new(Vec::new()),
        meshes:RefCell::new(Vec::new())
      }
  }

  pub fn clear(&self) {
    self.mesh_dic.borrow_mut().clear();
    self.material_dic.borrow_mut().clear();
    self.shader_dic.borrow_mut().clear();
    self.shaders.borrow_mut().clear();
    self.meterials.borrow_mut().clear();
    self.meshes.borrow_mut().clear();
  }

  pub fn push_node(&self,node:&RenderNode<B>) {
    if !self.shader_dic.borrow().contains_key(&node.material.get_shader_rc().id) {
      self.shaders.borrow_mut().push(Rc::clone(node.material.get_shader_rc()));
      self.shader_dic.borrow_mut().insert(node.material.get_shader_rc().id,self.shaders.borrow().len() - 1);
    }
    if !self.material_dic.borrow().contains_key(&node.material.id) {
      self.meterials.borrow_mut().push(Rc::clone(&node.material) );
      self.material_dic.borrow_mut().insert(node.material.id,self.meterials.borrow().len() - 1);
    }
    self.meshes.borrow_mut().push(Rc::clone(&node.mesh));
    self.mesh_dic.borrow_mut().insert(self.meshes.borrow().len() - 1, node.material.id);
  }

  pub fn get_mesh_material_by_idx(&self,idx:usize) -> u128 {
    self.mesh_dic.borrow().get(&idx).unwrap().clone()
  }
}