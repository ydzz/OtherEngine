use std::cell::{RefCell};
use crate::graphics::gfx_helper::{COLOR_RANGE,BufferState};
use crate::graphics::graphics::{Graphics};
use gfx_hal::{image as i,
              format::{AsFormat,Rgba8Srgb,Swizzle},device::{Device},
              memory as m,command,
              command::{CommandBuffer},
              format as f,pso,
              };
              
use gfx_hal::pso::{PipelineStage};
use gfx_hal::queue::{CommandQueue};
use image::GenericImageView;
use std::{iter};
use gfx_hal::pool::CommandPool;


pub struct Texture <B:gfx_hal::Backend> {
  pub image:image::DynamicImage,
  buffer:Option<BufferState<B>>,
  image_view:Option<B::ImageView>,
  sampler:Option<B::Sampler>
}

impl<B> Texture<B> where B:gfx_hal::Backend {
  pub fn load_by_path(path:&str) -> Texture<B> {
    let img = image::open(path).unwrap();

    Texture {image : img , buffer:None,image_view:None,sampler:None }
  }

  pub fn get_image_view_ref(&self) -> &B::ImageView {
    self.image_view.as_ref().unwrap()
  }

  pub fn get_sampler_ref(&self) -> &B::Sampler {
    self.sampler.as_ref().unwrap()
  }

  pub fn to_gpu(&mut self,gp:&RefCell<Graphics<B>>) {
    let (buffer,row_pitch) = unsafe { BufferState::new_texture(&gp.borrow().device,&self.image,&gp.borrow().memory_types) };
    self.buffer = Some(buffer);
    let stride = 4usize;
    let (width,height) = self.image.dimensions();
    let kind = i::Kind::D2(width as i::Size,height as i::Size, 1, 1);
    unsafe {
      let mut image = gp.borrow().device.borrow().create_image(kind,1,Rgba8Srgb::SELF,i::Tiling::Optimal,
                                                               i::Usage::TRANSFER_DST | i::Usage::SAMPLED,i::ViewCapabilities::empty()).unwrap();
      let req = gp.borrow().device.borrow().get_image_requirements(&image);
      let device_type = gp.borrow().memory_types.iter().enumerate().position(|(id, memory_type)| {
                req.type_mask & (1 << id) != 0
                    && memory_type.properties.contains(m::Properties::DEVICE_LOCAL)
            }).unwrap().into();
      let memory = gp.borrow().device.borrow().allocate_memory(device_type, req.size).unwrap();
      gp.borrow().device.borrow().bind_image_memory(&memory, 0, &mut image).unwrap();
      let image_view = gp.borrow().device.borrow().create_image_view(&image,i::ViewKind::D2,Rgba8Srgb::SELF,Swizzle::NO,COLOR_RANGE.clone()).unwrap();
      let sampler = gp.borrow().device.borrow().create_sampler(&i::SamplerDesc::new(i::Filter::Linear, i::WrapMode::Clamp)).expect("Can't create sampler");
      self.image_view = Some(image_view);
      self.sampler = Some(sampler);
      let mut transfered_image_fence = gp.borrow().device.borrow().create_fence(false).expect("Can't create fence");

      let mut cmd_buffer = gp.borrow_mut().command_pool.allocate_one(command::Level::Primary);
      cmd_buffer.begin_primary(command::CommandBufferFlags::ONE_TIME_SUBMIT);
      let image_barrier = m::Barrier::Image {
                states: (i::Access::empty(), i::Layout::Undefined)
                    .. (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone()};
      cmd_buffer.pipeline_barrier(
                pso::PipelineStage::TOP_OF_PIPE .. pso::PipelineStage::TRANSFER,
                m::Dependencies::empty(),
                &[image_barrier]);

      cmd_buffer.copy_buffer_to_image(
                self.buffer.as_ref().unwrap().get_buffer(),
                &image,
                i::Layout::TransferDstOptimal,
                &[command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: row_pitch / (stride as u32),
                    buffer_height: height as u32,
                    image_layers: i::SubresourceLayers {
                        aspects: f::Aspects::COLOR,
                        level: 0,
                        layers: 0 .. 1,
                    },
                    image_offset: i::Offset { x: 0, y: 0, z: 0 },
                    image_extent: i::Extent {
                        width:  width,
                        height: height,
                        depth: 1,
                    },
                }]);
      let image_barrier = m::Barrier::Image {
                states: (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal)
                    .. (i::Access::SHADER_READ, i::Layout::ShaderReadOnlyOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };
            cmd_buffer.pipeline_barrier(
                pso::PipelineStage::TRANSFER .. pso::PipelineStage::FRAGMENT_SHADER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

      cmd_buffer.finish();
      gp.borrow().queue_group.borrow_mut().queues[0].submit_without_semaphores(iter::once(&cmd_buffer),Some(&mut transfered_image_fence));
    }
    /*
    unsafe {
                              
      

      

      
      
      
      
      
      
      
      cmd_buffer.begin();
      let image_barrier = m::Barrier::Image {
                states: (i::Access::empty(), i::Layout::Undefined)
                    .. (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };

      cmd_buffer.pipeline_barrier(PipelineStage::TOP_OF_PIPE .. PipelineStage::TRANSFER,m::Dependencies::empty(),&[image_barrier]);
      let refbuf = self.buffer.as_ref().unwrap().get_buffer();
      cmd_buffer.copy_buffer_to_image(
                refbuf,
                &image,
                i::Layout::TransferDstOptimal,
                &[command::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_width: row_pitch / (stride as u32),
                    buffer_height: height as u32,
                    image_layers: i::SubresourceLayers {
                        aspects: f::Aspects::COLOR,
                        level: 0,
                        layers: 0 .. 1,
                    },
                    image_offset: i::Offset { x: 0, y: 0, z: 0 },
                    image_extent: i::Extent {
                        width: width,
                        height: height,
                        depth: 1,
                    },
                }],
            );

      let image_barrier = m::Barrier::Image {
                states: (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal)
                    .. (i::Access::SHADER_READ, i::Layout::ShaderReadOnlyOptimal),
                target: &image,
                families: None,
                range: COLOR_RANGE.clone(),
            };
      cmd_buffer.pipeline_barrier(PipelineStage::TRANSFER .. PipelineStage::FRAGMENT_SHADER,m::Dependencies::empty(),&[image_barrier]);
      cmd_buffer.finish();
      gp.borrow().queue_group.borrow_mut().queues[0].submit_without_semaphores(iter::once(&cmd_buffer),Some(&mut transfered_image_fence));
    }*/
  }
}

