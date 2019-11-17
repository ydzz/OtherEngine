use crate::graphics::gfx_helper::{Vertex3,DescSetLayout};
use crate::graphics::pipeline::Pipeline;
use crate::graphics::render_pass::RenderPass;
use crate::graphics::render_queue::QueueType;
use crate::graphics::shader::{compile_glsl_shader, Shader};
use gfx_hal::{
  device::Device, format as f, pass, pass::Subpass, pso, pso::VertexInputRate,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
extern crate glsl_to_spirv;
pub struct ShaderStore<B: gfx_hal::Backend> {
  shaders: HashMap<String, Rc<Shader<B>>>,
  device: Rc<RefCell<B::Device>>,
  render_pass: Rc<RenderPass<B>>,
}

impl<B> ShaderStore<B> where B: gfx_hal::Backend
{
  pub fn new(device: Rc<RefCell<B::Device>>, render_pass: Rc<RenderPass<B>>) -> Self {
    ShaderStore {
      shaders: HashMap::new(),
      device: device,
      render_pass: render_pass,
    }
  }

  pub fn init_builtin_shader(&mut self,mvp_layout:&DescSetLayout<B>) {
    let ui_shader = self.create_ui_builtin_shader(mvp_layout);
    self.shaders.insert(ui_shader.name.clone(), Rc::new(ui_shader));
  }

  pub fn get_shader(&self, shader_name: &str) -> &Rc<Shader<B>> {
    self.shaders.get(shader_name).unwrap()
  }


  fn create_ui_builtin_shader(&self,mvp_layout:&DescSetLayout<B>) -> Shader<B> {
    let desc_set_layout = DescSetLayout::new(Rc::clone(&self.device),vec![
      pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
      pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: pso::ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
      }
    ],10);
    let pipeline_layout = unsafe {
      self.device.borrow().create_pipeline_layout(
          vec![mvp_layout.raw_layout(),desc_set_layout.raw_layout()],
          &[(pso::ShaderStageFlags::VERTEX, 0..8)],
        ).unwrap()
    };
    let vert_code = compile_glsl_shader("resource/shader/ui.vert", glsl_to_spirv::ShaderType::Vertex).unwrap();
    let frag_code = compile_glsl_shader("resource/shader/ui.frag", glsl_to_spirv::ShaderType::Fragment).unwrap();
    let vs_module = unsafe { self.device.borrow().create_shader_module(&vert_code).unwrap() };
    let fs_module = unsafe { self.device.borrow().create_shader_module(&frag_code).unwrap() };
    let vs_entry: pso::EntryPoint<B> = pso::EntryPoint {
      entry: "main",
      module: &vs_module,
      specialization: pso::Specialization::default(),
    };
    let fs_entry: pso::EntryPoint<B> = pso::EntryPoint {
      entry: "main",
      module: &fs_module,
      specialization: pso::Specialization::default(),
    };
    let shader_entries = pso::GraphicsShaderSet {
      vertex: vs_entry,
      hull: None,
      domain: None,
      geometry: None,
      fragment: Some(fs_entry),
    };
    let ref_pass = self.render_pass.get_raw_pass();
    let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
      shader_entries,
      pso::Primitive::TriangleList,
      pso::Rasterizer::FILL,
      &pipeline_layout,
      pass::Subpass {
        index: 0,
        main_pass: ref_pass,
      },
    );

    pipeline_desc.blender.targets.push(pso::ColorBlendDesc {
      mask: pso::ColorMask::ALL,
      blend: Some(pso::BlendState::ALPHA),
    });

    pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
      binding: 0,
      stride: std::mem::size_of::<Vertex3>() as u32,
      rate: VertexInputRate::Vertex,
    });
   
    pipeline_desc.attributes.push(pso::AttributeDesc {
      location: 0,
      binding: 0,
      element: pso::Element {
        format: f::Format::Rg32Sfloat,
        offset: 0,
      },
    });
    pipeline_desc.attributes.push(pso::AttributeDesc {
      location: 1,
      binding: 0,
      element: pso::Element {
        format: f::Format::Rg32Sfloat,
        offset: 12,
      },
    });
    let raw_pipeline = unsafe {
      self
        .device
        .borrow()
        .create_graphics_pipeline(&pipeline_desc, None)
        .unwrap()
    };

    unsafe {
      self.device.borrow().destroy_shader_module(vs_module);
    }
    unsafe {
      self.device.borrow().destroy_shader_module(fs_module);
    }
    let pipeline: Pipeline<B> = Pipeline {
      device: Rc::clone(&self.device),
      desc_set_layout: RefCell::new(desc_set_layout),
      raw_pipeline: raw_pipeline,
      pipeline_layout: pipeline_layout,
    };
    Shader {
      id: uuid::Uuid::new_v4().as_u128(),
      name: String::from("UI"),
      pipelines: vec![pipeline],
      queue_type: QueueType::Geometry,
    }
  }

  pub fn re_create_render_pass(&mut self,pass:Rc<RenderPass<B>>,mvp_layout:&DescSetLayout<B>) {
    self.render_pass = pass;
    self.init_builtin_shader(mvp_layout);
  }
}
