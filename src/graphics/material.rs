use std::rc::{Rc};
use std::cell::{RefCell};
use gfx_hal as hal;
use gfx_hal::{device::{Device}, pso,image as i};
use crate::graphics::texture::{Texture};
use crate::graphics::shader::{Shader};

pub struct Material<B:hal::Backend> {
  pub id:u128,
  pub desc_set:B::DescriptorSet,
  shader:Rc<Shader<B>>,
  main_texture:Option<Rc<Texture<B>>>,
  device:Rc<RefCell<B::Device>>
}

impl<B> Material<B> where B:hal::Backend {
  pub fn new(shader:&Rc<Shader<B>>,device:&Rc<RefCell<B::Device>>) -> Self {
    let  pipeline = &shader.pipelines[0];
    let desc_set = pipeline.create_desc_set();
    
    Material {
        id:uuid::Uuid::new_v4().as_u128(),
        shader:Rc::clone(shader),
        desc_set:desc_set,
        main_texture:None,
        device:Rc::clone(device)
       }
  }

  pub fn get_shader(&self) -> &Rc<Shader<B>> {
    &self.shader
  }

  pub fn set_main_texture(&mut self,tex:Rc<Texture<B>>) {
    self.main_texture = Some(tex);
    unsafe { self.update_desc_set(); }
  }

  pub fn get_desc_set(&self) -> &B::DescriptorSet {
    &self.desc_set
  }

  unsafe fn  update_desc_set(&self) {
    self.main_texture.as_ref().map(|tex| {
      self.device.borrow().write_descriptor_sets(vec!(
        pso::DescriptorSetWrite { 
          set: &self.desc_set,
          binding: 0,
          array_offset: 0,
          descriptors: Some(pso::Descriptor::Image(tex.get_image_view_ref(),i::Layout::ShaderReadOnlyOptimal))
        },
        pso::DescriptorSetWrite {
                    set: &self.desc_set,
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(pso::Descriptor::Sampler(tex.get_sampler_ref())),
                }));
      
    });
  }

  
}