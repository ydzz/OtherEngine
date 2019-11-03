use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal as hal;
use crate::graphics::texture::{Texture};
use crate::graphics::shader::{Shader};

pub struct Material<B:hal::Backend> {
  pub id:u128,
  pub desc_set:B::DescriptorSet,
  shader:Rc<Shader<B>>,
  main_texture:Option<Texture<B>>
}

impl<B> Material<B> where B:hal::Backend {
  pub fn new(shader:&Rc<Shader<B>>) -> Self {
    let pipeline = &shader.pipelines[0];
    let desc_set = pipeline.create_desc_set();
    
    Material {
        id:uuid::Uuid::new_v4().as_u128(),
        shader:Rc::clone(shader),
        desc_set:desc_set,
        main_texture:None
       }
  }

  pub fn get_shader(&self) -> &Rc<Shader<B>> {
    &self.shader
  }

  pub fn set_main_texture(&mut self,tex:Texture<B>) {
    self.main_texture = Some(tex);
  }

  pub fn get_desc(&self) -> &B::DescriptorSet {
    &self.desc_set
  }

  
}