#![allow(dead_code, unused_extern_crates, unused_imports)]

extern crate gfx_backend_gl as back;
use gfx_hal::{
  format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
  window,
  Instance,
  window::Extent2D,
  adapter::{Adapter}
};
use winit::dpi::LogicalSize;
use winit::event_loop::{EventLoop};
use std::cell::{RefCell};
use std::rc::{Rc};

pub struct Win <T:IWinCall> {
  event_loop:EventLoop<()>,
  window:Option<winit::window::Window>,
  pub winsize:Extent2D,
  title:String,
  win_call:Option<Rc<RefCell<T>>>
}

impl<T> Win<T> where T:IWinCall {
 pub fn new() -> Self {
   let event_loop = EventLoop::new();
   Win {event_loop : event_loop,window : None,
        title:String::from("winit"), 
        winsize : Extent2D {width: 320,height: 240},win_call:None }
 }

 pub fn init(&mut self) -> (back::Surface,Adapter<back::Backend>) {
   let wb = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(1.0, 1.0))
        .with_inner_size(winit::dpi::LogicalSize::new(self.winsize.width as f64,self.winsize.height as f64))
        .with_title(self.title.clone());
   let builder = back::config_context(back::glutin::ContextBuilder::new(),ColorFormat::SELF, None).with_vsync(true);
   let windowed_context = builder.build_windowed(wb, &self.event_loop).unwrap();
   let (context,window) = unsafe { windowed_context.make_current().expect("Unable to make context current").split() };
   let surface = back::Surface::from_context(context);
   let mut adapters = surface.enumerate_adapters();
   
   let adapter = adapters.remove(0);
   self.window = Some(window);
   (surface,adapter)
 }

 pub fn set_win_call(&mut self,wc:Rc<RefCell<T>>) {
   self.win_call = Some(wc);
 }

 pub fn get_winsize(&self) -> Extent2D {
   self.winsize
 }


 pub fn on_resize(&mut self,w:f64,h:f64) {
   self.win_call.as_ref().map(|wc| {wc.borrow().resize(w,h) });
 }

 pub fn on_update(&mut self) {
   self.win_call.as_ref().map(|wc| {wc.borrow().update() });
 }

 pub fn run(&'static mut self) {
 
   self.event_loop.run(move |event, _, control_flow| {
    *control_flow = winit::event_loop::ControlFlow::Wait;
    match event {
      //WindowEvent
      winit::event::Event::WindowEvent { event, .. } => {
        match event {
          winit::event::WindowEvent::Resized(dims) => {
            self.on_resize(dims.width,dims.height);
          },
          winit::event::WindowEvent::RedrawRequested => {
            self.on_update();
          },
          winit::event::WindowEvent::CloseRequested => {
            *control_flow = winit::event_loop::ControlFlow::Exit
          }
        }
      },
      winit::event::Event::EventsCleared => {
        self.window.as_ref().map(|w| {w.request_redraw()});
      }
    }
   });
   /*
   while running  {
     self.event_loop.poll_events(|event| {
        if let winit::Event::WindowEvent { event, .. } = event { 
          match event {
            winit::WindowEvent::CloseRequested => {
              running = false
            },
            winit::WindowEvent::Resized(dims) => {
              recreate_swapchain = true;
              resize_dims.width = dims.width as u32;
              resize_dims.height = dims.height as u32;
              
            },
            _ => ()
          }
        }
     });
     if recreate_swapchain {
       resize_fn(resize_dims);
       recreate_swapchain = false;
     }
     draw_fn();
   }*/
 }
}

pub trait IWinCall {
  fn resize(&self,w:f64,h:f64);
  fn update(&self);
}