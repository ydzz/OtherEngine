use nalgebra::{Matrix4,Isometry3,Vector3};


pub struct Transform {
 isometry:Isometry3<f32>,
 scale:Vector3<f32>,
 world_mat:Matrix4<f32>
}

impl Transform {
 pub fn identity () ->Transform {
   Transform {mat : nalgebra::one() }
 }

 pub fn get_position(&self) -> Vector3<f32> {
   self.isometry3.translation.vector
 }

 pub fn matrix(&self) -> Matrix4<f32> {
   self.isometry.to_homogeneous().prepend_nonuniform_scaling(&self.scale)
 }


}