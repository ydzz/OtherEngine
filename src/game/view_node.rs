use crate::graphics::transform::Transform;
use nalgebra::Vector3;
use std::collections::HashMap;
use crate::graphics::{RenderNode,Texture};
pub enum ViewValue {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Vec3(Vector3<f32>),
    Texture(Texture)
}
pub struct ViewNode {
    v_type: String,
    transform: Transform,
    property: HashMap<String, ViewValue>,
    
    node:Option<RenderNode>,
    children:Vec<ViewNode>
}

pub fn vnode(vtype:String,param:HashMap<String, ViewValue>,children:Vec<ViewNode>) -> ViewNode {
   ViewNode {
       v_type : vtype,
       transform : Transform::identity(),
       property : param,
       children : children,
       node : None
   }
}

pub fn image(param:HashMap<String,ViewValue>) -> ViewNode {
    vnode(String::from("Image"),param, Vec::new())
}