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
/*
fn main() {
   let rt  = JSRuntime::new().unwrap();
   let ctx = JSContext::new(&rt).unwrap();
   let refcell = RefCell::new(ctx);
   init_internal(&refcell);
   init_fs_binding(&refcell);

   let filecode = std::fs::read("main.js").unwrap();
   let str:String = String::from_utf8(filecode).unwrap();
   refcell.borrow().eval(&str, "main.js", EvalType::Module as i32);

   //create window
   use glium::{glutin, Surface};
   let mut events_loop = glium::glutin::EventsLoop::new();
   let wb = glium::glutin::WindowBuilder::new();

   let cb = glium::glutin::ContextBuilder::new();
   let display = glium::Display::new(wb, cb, &events_loop).unwrap();

   let mut closed = false;
   while !closed {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
   }
}
*/