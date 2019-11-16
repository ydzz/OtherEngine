#![allow(dead_code, unused_extern_crates, unused_imports)]
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(not(any(
    feature = "gl",
    feature = "dx12"
)))]
extern crate gfx_backend_vulkan as back;

use gfx_hal::{
  format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
  window,
  Instance,
  window::Extent2D,
  adapter::{Adapter}
};
use crate::graphics::gfx_helper::{GPBackend};
use winit::dpi::LogicalSize;
use winit::event_loop::{EventLoop};
use std::cell::{RefCell};
use std::rc::{Rc};

pub struct Win <T:IWinCall + 'static> {
  event_loop:Option<EventLoop<()>>,
  window:Option<winit::window::Window>,
  pub winsize:Extent2D,
  title:String,
  win_call:Option<Rc<RefCell<T>>>
}

impl<T> Win<T> where T:IWinCall {
 pub fn new() -> Self {
   let event_loop = EventLoop::new();
   Win {event_loop : Some(event_loop),window : None,
        title:String::from("winit"), 
        winsize : Extent2D {width: 174,height: 166},win_call:None }
 }

 #[cfg(feature = "gl")]
 pub fn init<B:gfx_hal::Backend>(&mut self) -> GPBackend<back::Backend> {
   let wb = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(1.0, 1.0))
        .with_inner_size(winit::dpi::LogicalSize::new(self.winsize.width as f64,self.winsize.height as f64))
        .with_title(self.title.clone());
   let builder = back::config_context(back::glutin::ContextBuilder::new(),ColorFormat::SELF, None).with_vsync(true);
   let windowed_context = builder.build_windowed(wb, self.event_loop.as_ref().unwrap()).unwrap();
   let (context,window) = unsafe { windowed_context.make_current().expect("Unable to make context current").split() };
   let surface = back::Surface::from_context(context);
   let mut adapters = surface.enumerate_adapters();
   
   let adapter = adapters.remove(0);
   self.window = Some(window);
   GPBackend {surface : surface , adapter:adapter}
 }

 #[cfg(not(any(
  feature = "gl",
  feature = "dx12"
)))]
pub fn init(&mut self) -> GPBackend<back::Backend> {
  let wb = winit::window::WindowBuilder::new()
        .with_min_inner_size(winit::dpi::LogicalSize::new(1.0, 1.0))
        .with_inner_size(winit::dpi::LogicalSize::new(self.winsize.width as f64,self.winsize.height as f64))
        .with_title(self.title.clone());
  let window = wb.build(self.event_loop.as_ref().unwrap()).expect("init window fail");
  self.window = Some(window);
  let instance = back::Instance::create(self.title.as_str(), 1).expect("Failed to create an instance!");
  let surface = unsafe {
    instance.create_surface(self.window.as_ref().unwrap()).expect("Failed to create a surface!")
  };
  let mut adapters = instance.enumerate_adapters();
  GPBackend {surface : surface , adapter:adapters.remove(0)}
}

 pub fn set_win_call(&mut self,wc:Rc<RefCell<T>>) {
   self.win_call = Some(wc);
 }

 pub fn get_winsize(&self) -> Extent2D {
   self.winsize
 }

 pub fn run(&mut self) {
   let eloop = self.event_loop.take().unwrap();
   let win_call = self.win_call.take().unwrap();
   let window = self.window.take().unwrap();
   eloop.run(move |event, _, control_flow| {
    *control_flow = winit::event_loop::ControlFlow::Wait;
    let inner_window = &window;
    let inner_win_call = &win_call;
    match event {
      //WindowEvent
      winit::event::Event::WindowEvent { event, .. } => {
        match event {
          winit::event::WindowEvent::Resized(dims) => {
            inner_win_call.borrow().resize(dims.width,dims.height);
          },
          winit::event::WindowEvent::RedrawRequested => {
            inner_win_call.borrow().call_update();
          },
          winit::event::WindowEvent::CloseRequested => {
            *control_flow = winit::event_loop::ControlFlow::Exit
          },
          _=>(),
        }
      },
      winit::event::Event::EventsCleared => {
        inner_window.request_redraw();
      },
      _=>()
    }
   });
 }
}

pub trait IWinCall {
  fn resize(&self,w:f64,h:f64);
  fn call_update(&self);
}