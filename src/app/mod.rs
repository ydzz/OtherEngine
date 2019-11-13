//use qjs_rs::{JSContext,JSRuntime,init_internal,EvalType};
extern crate gfx_backend_gl as back;
use crate::win::{Win};
use crate::graphics::{Graphics,RenderNode,Material};
use crate::graphics::transform::{Transform};
use crate::graphics::texture::{Texture};
use crate::graphics::camera::{Camera};
use crate::game::{Game};
use std::cell::{RefCell};
use std::rc::{Rc};
use alga;
use nalgebra::{Vector3,Isometry3,Translation3,Matrix4};
//use crate::jsbinding::fs::init_fs_binding;
pub struct App {
  //js_ctx:RefCell<JSContext>,
  //js_rt:JSRuntime
  game:RefCell<Game>,
  window:RefCell<Win>,
}

impl App {
  pub fn new() -> Self {
    //let rt  = JSRuntime::new().unwrap();
    //let ctx = JSContext::new(&rt).unwrap();
    //let ref_ctx = RefCell::new(ctx);
    let mut win = Win::new();
    let start = chrono::Local::now();
    let (surface,adapter) = win.init();
    let end = chrono::Local::now();
    println!("newapp {} ms",end.timestamp_millis() - start.timestamp_millis());
    let  graphics = Graphics::new(surface,adapter,win.get_winsize());
    let  game = Game::new(graphics);
    App { window : RefCell::new(win), game : RefCell::new(game)  /* js_rt:rt,js_ctx:ref_ctx*/ }
  }

/*
  pub fn init_js_engine(&self) {
    println!("init_js_engine");
    init_internal(&self.js_ctx);
    init_fs_binding(&self.js_ctx);

    self.js_ctx.borrow().eval("console.log('123')", "main.js", EvalType::Module as i32);
  }
*/
 
  pub fn run(&mut self) {
    //let test_node = create_test_node(&self.graphics);
    let size = self.window.borrow().get_winsize();
    let camera = Camera::standard_2d(size.width as f32, size.height as f32);
    self.game.borrow_mut().add_camera(camera);
  



    self.window.borrow_mut().run(|newsize| {
      self.game.borrow().resize_view(newsize);
    },||{
      self.game.borrow().update();

    });
  }
}

fn create_test_node<B:gfx_hal::Backend>(graphics:&RefCell<Graphics>) -> RenderNode {
    
    let rc_mesh = Rc::clone(&graphics.borrow().mesh_store.quad);
    let start0 = chrono::Local::now();
    let mut tex = Texture::load_by_path("resource/logo.png");
    
    let end = chrono::Local::now();
    println!("to gpu {} ms",end.timestamp_millis() - start0.timestamp_millis());
    tex.to_gpu(&graphics);
    
    let mut mat = Material::new(graphics.borrow().shader_store.get_shader("UI"),&graphics.borrow().device);
    mat.set_main_texture(tex);
    let mut node_t = Matrix4::identity();
  
    RenderNode {
      mat4:node_t,
      mesh :  rc_mesh,
      material : Rc::new(mat)
    }
}