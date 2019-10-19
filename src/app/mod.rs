use qjs_rs::{JSContext,JSRuntime,init_internal,EvalType};
use std::cell::RefCell;

use crate::jsbinding::fs::init_fs_binding;
pub struct App {
  js_ctx:RefCell<JSContext>,
  js_rt:JSRuntime
}

impl App {
  pub fn new() -> Self {
    let rt  = JSRuntime::new().unwrap();
    let ctx = JSContext::new(&rt).unwrap();
    let ref_ctx = RefCell::new(ctx);
    App { js_rt:rt,js_ctx:ref_ctx }
  }

  pub fn init_js_engine(&self) {
    println!("init_js_engine");
    init_internal(&self.js_ctx);
    init_fs_binding(&self.js_ctx);

    self.js_ctx.borrow().eval("console.log('123')", "main.js", EvalType::Module as i32);

  
  }
 
  pub fn run(&self) {
    println!("run");
    self.init_js_engine()


  }
}