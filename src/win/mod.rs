#![allow(dead_code, unused_extern_crates, unused_imports)]

extern crate gfx_backend_gl as back;
use gfx_hal::{
  format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle},
  window,
  Instance,
  Adapter
};

use winit::dpi::LogicalSize;

pub struct Win {
  event_loop:winit::EventsLoop,
  window:Option<winit::Window>,
  title:String
}

impl Win {
 pub fn new() -> Self {
   let event_loop = winit::EventsLoop::new();
   Win {event_loop : event_loop,window : None,title:String::from("winit") }
 }

 pub fn init(&mut self) -> (back::Surface,Adapter<back::Backend>) {
   let wb = winit::WindowBuilder::new()
        .with_min_dimensions(winit::dpi::LogicalSize::new(1.0, 1.0))
        .with_dimensions(winit::dpi::LogicalSize::new(320_f64,240_f64))
        .with_title(self.title.clone());
   let builder = back::config_context(back::glutin::ContextBuilder::new(),ColorFormat::SELF, None).with_vsync(true);
   let windowed_context = builder.build_windowed(wb, &self.event_loop).unwrap();
   let (context,window) = unsafe { windowed_context.make_current().expect("Unable to make context current").split() };
   let surface = back::Surface::from_context(context);
   let mut adapters = surface.enumerate_adapters();
   for adapter in &adapters {
        println!("{:?}", adapter.info);
   }
   let adapter = adapters.remove(0);
   self.window = Some(window);
   (surface,adapter)
 }

 pub fn run(&mut self) {
   let mut running  = true;
   //let mut recreate_swapchain = false;
   while running  {
     self.event_loop.poll_events(|event| {
        if let winit::Event::WindowEvent { event, .. } = event { 
          match event {
            winit::WindowEvent::CloseRequested => {
              running = false
            },
            _ => ()
          }
        }
     });

   }
 }
}