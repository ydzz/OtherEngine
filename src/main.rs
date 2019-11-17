//mod jsbinding;
mod app;
mod win;
mod graphics;
mod game;
use app::{App};

//use jsbinding::fs::init_fs_binding;
fn main () {
  let mut app = App::new();
  app.run()
}