extern crate gfx_backend_gl as back;
use crate::graphics::pipeline::{Pipeline};
extern crate byteorder;
use std::string::String;
use std::ops::Add;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

pub struct Shader<B:gfx_hal::Backend> {
 pub pipelines:Vec<Pipeline<B>>
}

impl <B> Shader<B> where B:gfx_hal::Backend {
  
}

pub fn compile_glsl_shader(file_path:&str,shader_type:glsl_to_spirv::ShaderType) -> Result<Vec<u32>,String> {
   let spv_path = String::from(file_path).add(".spv");
   let spv_file = std::fs::read(&spv_path);
   if spv_file.is_ok() {
     let spv_byte = spv_file.unwrap();
     let mut v32:Vec<u32> = Vec::new();
     for i in (0..spv_byte.len()).step_by(4) {
        v32.push(LittleEndian::read_u32(&spv_byte[i..]));
     }
     return Ok(v32);
   }
   let file_str = std::fs::read_to_string(file_path).map_err(|_| { String::from("read shader fail") })?;
   let vert_file = glsl_to_spirv::compile(&file_str, shader_type)?;
   let spv = gfx_hal::read_spirv(vert_file).unwrap();
   let mut v8: Vec<u8> = Vec::new();
   for n in &spv {
        v8.write_u32::<LittleEndian>(*n).unwrap();
   }
   std::fs::write(&spv_path, v8).unwrap();
   Ok(spv)
}

