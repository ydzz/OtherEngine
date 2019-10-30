extern crate gfx_backend_gl as back;
use crate::graphics::pipeline::{Pipeline};
pub struct Shader<B:gfx_hal::Backend> {
 pub pipelines:Vec<Pipeline<B>>
}

impl <B> Shader<B> where B:gfx_hal::Backend {
  
}

pub fn compile_glsl_shader(file_path:&str,shader_type:glsl_to_spirv::ShaderType) -> Result<Vec<u32>,String> {
   let file_str = std::fs::read_to_string(file_path).map_err(|_| { String::from("read shader fail") })?;
   let vert_file = glsl_to_spirv::compile(&file_str, shader_type)?;
   let spv = gfx_hal::read_spirv(vert_file).unwrap();
   Ok(spv)
}

