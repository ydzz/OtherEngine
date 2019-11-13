use crate::graphics::transform::Transform;
use nalgebra::Vector3;
use std::collections::HashMap;
pub enum ViewValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Vec3(Vector3<f32>),
}
pub struct ViewNode {
    v_type: String,
    transform: Transform,
    property: HashMap<String, ViewValue>,
}
