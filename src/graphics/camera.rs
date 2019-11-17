use nalgebra::{Matrix4};
use crate::graphics::transform::{Transform};
use alga;
#[derive(Debug, Copy, Clone)]
pub struct Orthographic {
    matrix: Matrix4<f32>,
    inverse_matrix: Matrix4<f32>,
}

impl PartialEq for Orthographic {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl Orthographic {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Self {
        let mut matrix = Matrix4::<f32>::identity();
        matrix[(0, 0)] = 2.0 / (right - left);
        matrix[(1, 1)] = -2.0 / (top - bottom);
        matrix[(2, 2)] = -1.0 / (z_far - z_near);
        matrix[(0, 3)] = -(right + left) / (right - left);
        matrix[(1, 3)] = -(top + bottom) / (top - bottom);
        matrix[(2, 3)] = -z_near / (z_far - z_near);
        Self {
            matrix,
            inverse_matrix: matrix
                .try_inverse()
                .expect("Camera projection matrix is not invertible. This is normally due to having inverse values being superimposed (near=far, right=left)"),
        }
    }

    #[inline]
    pub fn as_matrix(&self) -> &Matrix4<f32> {
        &self.matrix
    }
    #[inline]
    pub fn as_inverse_matrix(&self) -> &Matrix4<f32> {
        &self.inverse_matrix
    }
}

#[derive(Clone, Debug,PartialEq)]
pub enum Projection {
    Orthographic(Orthographic)
}

impl Projection {
    pub fn as_matrix(&self) -> &Matrix4<f32> {
       match *self {
         Projection::Orthographic(ref orth) => orth.as_matrix()
       }
    }
}

pub struct Camera {
    pub transform:Transform,
    projection:Projection
}

impl Camera {
    pub fn standard_2d(width: f32, height: f32) -> Self {
        Camera {
            transform:Transform::identity(),
            projection : Projection::Orthographic(Orthographic::new(-width / 2.0,
                width / 2.0,
                height / 2.0,
                -height / 2.0,
                -2000.0,
                2000.0))
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
       self.transform.view_matrix()
    }

    pub fn p_matrix(&self) -> &Matrix4<f32> {
        self.projection.as_matrix()
    }
}