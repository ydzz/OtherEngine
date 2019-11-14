pub mod pool;
pub mod image;
mod view_node;
use std::rc::{Rc};
use nalgebra::{Matrix4,Vector3};
use crate::graphics::camera::Camera;
use crate::graphics::transform::{Transform};
use crate::graphics::{Graphics,Texture,RenderNode,Material};
use gfx_hal::window::Extent2D;
use std::collections::{HashMap};
use std::cell::RefCell;
use view_node::{ViewNode,image,ViewValue};
use crate::win::IWinCall;
pub struct Game {
    graphics: RefCell<Graphics>,
    camera_list: Vec<Camera>,
    view_tree:Option<ViewNode>,
    render_list:Vec<Rc<RenderNode>>
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        Game {
            graphics: RefCell::new(graphics),
            camera_list: Vec::new(),
            view_tree: None,
            render_list: Vec::new()
        }
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.camera_list.push(camera);
    }

    pub fn init_view(&mut self) {
        let mut tex = Texture::load_by_path("resource/logo.png");
        tex.to_gpu(&self.graphics);
        let mut map = HashMap::new();
        map.insert(String::from("Texture"),ViewValue::Texture(tex) );
        map.insert(String::from("Width"),ViewValue::Int(100) );
        map.insert(String::from("Height"),ViewValue::Int(100) );
        self.view_tree = Some(image(map));

        let test_node = Rc::new(create_test_node(&self.graphics,100f32));
        self.render_list.push(test_node);

        let test_node = Rc::new(create_test_node(&self.graphics,-100f32));
        self.render_list.push(test_node);
    }

    pub fn update(&self) {
        self.graphics.borrow_mut().draw(&self.camera_list, &self.render_list);
    }

    pub fn resize_view(&self, size: Extent2D) {
        self.graphics.borrow_mut().recreate_swapchain(size);
    }
}



fn create_test_node(graphics:&RefCell<Graphics>,xoffset:f32) -> RenderNode {
    
    let rc_mesh = Rc::clone(&graphics.borrow().mesh_store.quad);

    let mut tex = Texture::load_by_path("resource/logo.png");
    tex.to_gpu(&graphics);
    
    let mut mat = Material::new(graphics.borrow().shader_store.get_shader("UI"),&graphics.borrow().device);
    mat.set_main_texture(tex);
    let mut node_t = Transform::identity();
    node_t.set_scale(Vector3::new(100f32,100f32,100f32));
    node_t.set_x(xoffset);
    RenderNode {
      mat4:node_t.matrix(),
      mesh :  rc_mesh,
      material : Rc::new(mat)
    }
}

impl IWinCall for Game {
  fn resize(&self,w:f64,h:f64) {
     self.graphics.borrow_mut().recreate_swapchain(Extent2D {width : w as u32, height : h as u32})
  }

  fn call_update(&self) {
      self.update();
  }
}