use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal as hal;
use crate::graphics::shader::{Shader};
use crate::graphics::shader_store::{ShaderStore};

pub struct Material<B:hal::Backend> {
  pub id:u128,
  pub shader_name:String,
  pub desc_set:Option<B::DescriptorSet>,
  shader_store:Rc<ShaderStore<B>>
}

impl<B> Material<B> where B:hal::Backend {
  pub fn new(shader_store:&Rc<ShaderStore<B>>,shader:String) -> Self {
    let pipeline = &shader_store.get_shader(&shader).pipelines[0];
    //let desc = pipeline.create_desc_set();
    Material {
        id:uuid::Uuid::new_v4().as_u128(),
        shader_name:shader,
        desc_set:None,
        shader_store:Rc::clone(shader_store)
       }
  }

  pub fn get_shader_rc(&self) -> &Rc<Shader<B>> {
    self.shader_store.get_shader(&self.shader_name)
  }
}