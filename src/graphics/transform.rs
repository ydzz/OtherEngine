use nalgebra::{Isometry3, Matrix4, Vector3};

pub struct Transform {
  isometry: Isometry3<f32>,
  scale: Vector3<f32>,
  world_mat: Matrix4<f32>,
}

impl Transform {
  pub fn identity() -> Transform {
    Transform {
      isometry: Isometry3::identity(),
      scale: Vector3::new(1f32, 1f32, 1f32),
      world_mat: Matrix4::identity(),
    }
  }

  pub fn get_position(&self) -> Vector3<f32> {
    self.isometry.translation.vector
  }

  pub fn matrix(&self) -> Matrix4<f32> {
    self.isometry.to_homogeneous().prepend_nonuniform_scaling(&self.scale)
  }

  pub fn view_matrix(&self) -> Matrix4<f32> {
    let inv_scale = Vector3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
    self.isometry
        .inverse()
        .to_homogeneous()
        .append_nonuniform_scaling(&inv_scale)
  }

 

  pub fn set_scale(&mut self, scale: Vector3<f32>) {
    self.scale = scale;
  }

  pub fn set_x(&mut self, value: f32) -> &mut Self {
    self.isometry.translation.vector.x = value;
    self
  }

  pub fn set_y(&mut self, value: f32) -> &mut Self {
    self.isometry.translation.vector.y = value;
    self
  }
  
  pub fn set_z(&mut self, value: f32) -> &mut Self {
    self.isometry.translation.vector.z = value;
    self
  }
}
