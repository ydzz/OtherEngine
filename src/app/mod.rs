//use qjs_rs::{JSContext,JSRuntime,init_internal,EvalType};
extern crate gfx_backend_gl as back;
use crate::win::{Win};
use crate::graphics::{Graphics};
//use crate::jsbinding::fs::init_fs_binding;
pub struct App {
  //js_ctx:RefCell<JSContext>,
  //js_rt:JSRuntime
  graphics:Graphics<back::Backend>,
  window:Win
}

impl App {
  pub fn new() -> Self {
    //let rt  = JSRuntime::new().unwrap();
    //let ctx = JSContext::new(&rt).unwrap();
    //let ref_ctx = RefCell::new(ctx);

    let mut win = Win::new();
    let (surface,adapter) = win.init();
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
    let ref_graphics = &mut self.graphics;
    self.window.run(|newsize| {
      ref_graphics.recreate_swapchain(newsize);
    });

  }
}