//use qjs_rs::{JSContext,JSRuntime,init_internal,EvalType};
extern crate gfx_backend_gl as back;
use crate::win::{Win};
use crate::graphics::{Graphics};
use crate::graphics::render_node::{RenderNode};
use crate::graphics::material::{Material};
use crate::graphics::texture::{Texture};
use std::cell::{RefCell};
use std::rc::{Rc};
//use crate::jsbinding::fs::init_fs_binding;
pub struct App {
  //js_ctx:RefCell<JSContext>,
  //js_rt:JSRuntime
  graphics:RefCell<Graphics<back::Backend>>,
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
    App { window : RefCell::new(win),graphics : RefCell::new(graphics) /* js_rt:rt,js_ctx:ref_ctx*/ }
  }

/*
  pub fn init_js_engine(&self) {
    println!("init_js_engine");
    init_internal(&self.js_ctx);
    init_fs_binding(&self.js_ctx);

    self.js_ctx.borrow().eval("console.log('123')", "main.js", EvalType::Module as i32);
  }
*/
 
  pub fn run(&self) {
    
    let test_node = create_test_node(&self.graphics);
    let vec = vec!(&test_node); 

    self.graphics.borrow_mut().draw(&vec);
    self.window.borrow_mut().run(|newsize| {
      let (w,h) = self.graphics.borrow().get_winsize();
      if w != newsize.width && h != newsize.height {
       self.graphics.borrow_mut().recreate_swapchain(newsize);
      }
    },||{
      self.graphics.borrow_mut().draw(&vec);
    });
  }
}

fn create_test_node<B:gfx_hal::Backend>(graphics:&RefCell<Graphics<B>>) -> RenderNode<B> {
    
    let rc_mesh = Rc::clone(&graphics.borrow().mesh_store.quad2d);
    let start0 = chrono::Local::now();
    let mut tex = Texture::load_by_path("resource/logo.png");
    
    tex.to_gpu(&graphics);
    let end = chrono::Local::now();
    println!("to gpu {} ms",end.timestamp_millis() - start0.timestamp_millis());
    
    
    let mut mat = Material::new(graphics.borrow().shader_store.get_shader("UI"));
    mat.set_main_texture(tex);
    
    
    RenderNode { 
      mesh :  rc_mesh,
      material : Rc::new(mat)
    }
}