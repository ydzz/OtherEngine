//use qjs_rs::{JSContext,JSRuntime,init_internal,EvalType};
extern crate gfx_backend_gl as back;
use crate::win::{Win};
use crate::graphics::{Graphics};
use crate::graphics::render_node::{RenderNode};
use crate::graphics::material::{Material};
use std::cell::{RefCell};
use std::rc::{Rc};
//use crate::jsbinding::fs::init_fs_binding;
pub struct App {
  //js_ctx:RefCell<JSContext>,
  //js_rt:JSRuntime
  graphics:Graphics<back::Backend>,
  window:Win,
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
    
    App { window : win ,graphics : graphics /* js_rt:rt,js_ctx:ref_ctx*/ }
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
    let refcell = RefCell::new(&mut self.graphics);
    let rc_mesh = Rc::clone(&refcell.borrow().mesh_store.quad2d);
    let test_node = RenderNode { 
      mesh :  rc_mesh,
      material : Rc::new(Material::new(&refcell.borrow().shader_store,String::from( "UI")))
    };
   let vec = vec!(&test_node);
    
    refcell.borrow_mut().draw(&vec);
    self.window.run(|newsize| {
      let (w,h) = refcell.borrow().get_winsize();
      if w != newsize.width && h != newsize.height {
       refcell.borrow_mut().recreate_swapchain(newsize);
      }
    },||{
      refcell.borrow_mut().draw(&vec);
    });

  }
}