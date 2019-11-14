//use qjs_rs::{JSContext,JSRuntime,init_internal,EvalType};
extern crate gfx_backend_gl as back;
use crate::win::{Win};
use crate::graphics::camera::{Camera};
use crate::graphics::{Graphics};
use crate::game::{Game};
use std::cell::{RefCell};
use std::rc::{Rc};
//use crate::jsbinding::fs::init_fs_binding;
pub struct App {
  //js_ctx:RefCell<JSContext>,
  //js_rt:JSRuntime
  game:Rc<RefCell<Game>>,
  window:Win<Game>,
}

impl App {
  pub fn new() -> Self {
    //let rt  = JSRuntime::new().unwrap();
    //let ctx = JSContext::new(&rt).unwrap();
    //let ref_ctx = RefCell::new(ctx);
    let mut win:Win<Game> = Win::new();
    let start = chrono::Local::now();
    let (surface,adapter) = win.init();
    let end = chrono::Local::now();
    println!("newapp {} ms",end.timestamp_millis() - start.timestamp_millis());
    let  graphics = Graphics::new(surface,adapter,win.get_winsize());
    let  mut game = Game::new(graphics);
    game.init_view();
    let game_rc = Rc::new(RefCell::new(game)) ;
    win.set_win_call(Rc::clone(&game_rc));
    App { window : win , game : game_rc /* js_rt:rt,js_ctx:ref_ctx*/ }
  }

/*
  pub fn init_js_engine(&self) {
    println!("init_js_engine");
    init_internal(&self.js_ctx);
    init_fs_binding(&self.js_ctx);

    self.js_ctx.borrow().eval("console.log('123')", "main.js", EvalType::Module as i32);
  }
*/
 
  pub fn run(&'static mut self) {
    let size = self.window.get_winsize();
    let camera = Camera::standard_2d(size.width as f32, size.height as f32);
    self.game.borrow_mut().add_camera(camera);

    self.window.run();
  }
}
