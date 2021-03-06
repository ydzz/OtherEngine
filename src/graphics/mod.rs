#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(not(any(
    feature = "gl",
    feature = "dx12"
)))]
extern crate gfx_backend_vulkan as back;

mod graphics;
pub mod pipeline;
pub mod mesh_store;
pub mod gfx_helper;
mod shader;
pub mod shader_store;
pub mod render_pass;
mod render_node;
mod mesh;
mod material;
pub mod render_queue;
pub mod texture;
pub mod camera;
pub mod transform;

pub type RenderNode = render_node::RenderNode<back::Backend>;
pub type Graphics = graphics::Graphics<back::Backend>;
pub type Mesh = mesh::Mesh<back::Backend>;
pub type Shader = shader::Shader<back::Backend>;
pub type Material = material::Material<back::Backend>;
pub type Texture = texture::Texture<back::Backend>;