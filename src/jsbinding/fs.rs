use qjs_rs::{CJSValue,JSContext,JSPropertyItem,JSValue};
use qjs_rs::q;
use std::fs;
use std::collections::HashMap;

use std::cell::RefCell;
pub fn init_fs_binding(ctx:&RefCell<JSContext>) {
 let prop_list = vec!(
                       JSPropertyItem::func("exists",Some(export_exists), 1),
                       JSPropertyItem::func("mkdir",Some(export_mkdir), 2),
                       JSPropertyItem::func("readdir",Some(export_read_dir), 2),
                       JSPropertyItem::func("rename",Some(export_rename), 2),
                       JSPropertyItem::func("readFile",Some(export_read_file), 2),
                       JSPropertyItem::func("writeFile",Some(export_write_file), 2),
                       JSPropertyItem::func("unlink",Some(export_unlink), 2),
                       JSPropertyItem::func("rmdir",Some(export_rmdir), 2),
                     );
  ctx.borrow_mut().add_cmodule("fs", &prop_list);
}


pub extern "C" fn export_exists( ctx: *mut q::JSContext, _: q::JSValue, _: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
       let path_str = unsafe { CJSValue::deserialize_value(*argv, ctx).unwrap().to_string() };
       let metadata = fs::metadata(path_str);
       match metadata {
         Err(_) => CJSValue::val_bool(false),
         Ok(_) => CJSValue::val_bool(true),
       }
}

pub extern "C" fn export_mkdir( ctx: *mut q::JSContext, _: q::JSValue, argc: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
       let path_str = unsafe { CJSValue::deserialize_value(*argv, ctx).unwrap().to_string() };
       let mut is_recursive = false;
       if argc > 1 {
          let arg_slice = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
          let bval = CJSValue::deserialize_value(arg_slice[1], ctx).unwrap();
          is_recursive = bval.as_bool();
       }
       if is_recursive {
          let ret = fs::create_dir_all(path_str);
          CJSValue::val_bool(ret.is_ok())
       } else {
          let ret = fs::create_dir(path_str);
          CJSValue::val_bool(ret.is_ok())
       }
}

pub extern "C" fn export_read_dir( ctx: *mut q::JSContext, _: q::JSValue, argc: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
    let path_str = unsafe { CJSValue::deserialize_value(*argv, ctx).unwrap().to_string() };
    let mut is_deatil = false;
    if argc > 1 {
       let arg_slice = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
       let bval = CJSValue::deserialize_value(arg_slice[1], ctx).unwrap();
       is_deatil = bval.as_bool();
    }
    let res = fs::read_dir(path_str);
    if res.is_err() {
       return CJSValue::val_null();
    }
    let mut ret_array:Vec<JSValue> = Vec::default();
    for item in res.unwrap() {
       if item.is_err() {
          continue;
       }
       let item_v:fs::DirEntry = item.unwrap();
       let file_path = item_v.path().into_os_string().into_string().unwrap();
       let file_name = item_v.file_name().into_string().unwrap();
       if is_deatil {
         let mut detail_map:HashMap<String,JSValue> = HashMap::default();
         let may_detail =  item_v.metadata();
         if may_detail.is_err() { continue };
         let detail = may_detail.unwrap();
         detail_map.insert(String::from("isFile"), JSValue::Bool(detail.is_file()));
         detail_map.insert(String::from("isDirectory") , JSValue::Bool(detail.is_dir()));
         detail_map.insert(String::from("name"), JSValue::String(file_name));
         detail_map.insert(String::from("path"), JSValue::String(file_path));
         ret_array.push(JSValue::Object(detail_map));
       } else {
          ret_array.push(JSValue::String(file_path));
       }
    }
    JSValue::Array(ret_array).to_c_value(ctx)
}

pub extern "C" fn export_rename(ctx: *mut q::JSContext, _: q::JSValue, argc: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
   if argc != 2 {
      eprintln!("[Error] fs.rename must have 2 arg");
      return CJSValue::val_bool(false);
   };
   let arg_slice = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
   let oldpath = CJSValue::deserialize_value(arg_slice[0], ctx).unwrap().to_string();
   let newpath = CJSValue::deserialize_value(arg_slice[1], ctx).unwrap().to_string();
   let ret = fs::rename(oldpath, newpath);
   CJSValue::val_bool(ret.is_ok())
}

pub extern "C" fn export_read_file(ctx: *mut q::JSContext, _: q::JSValue, argc: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {

   let arg_slice = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
   let filepath = CJSValue::deserialize_value(arg_slice[0], ctx).unwrap().to_string();
   let mut is_byte = false;
   if argc > 1 {
      is_byte = CJSValue::deserialize_value(arg_slice[1], ctx).unwrap().as_bool();
   }
   if is_byte {
     let rfile = fs::read(&filepath);
     if rfile.is_err() { eprintln!("[Error] read file {} error",filepath); return CJSValue::val_null() }
     CJSValue::val_array_buffer(rfile.unwrap(),ctx)
   } else {
     let rfile = fs::read_to_string(&filepath);
     if rfile.is_err() { eprintln!("[Error] read file {} error",filepath); return CJSValue::val_null() }
     CJSValue::val_string(&rfile.unwrap(),ctx)
   }
}

pub extern "C" fn export_write_file(ctx: *mut q::JSContext, _: q::JSValue, argc: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
   assert_eq!(argc,2,"fs.wirteFile argc != 2");
   let arg_slice = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
   let filepath = CJSValue::deserialize_value(arg_slice[0], ctx).unwrap().to_string();
   if arg_slice[1].tag == q::JS_TAG_STRING.into() {
     let str_buf = CJSValue::deserialize_value(arg_slice[1], ctx).unwrap().to_string();
     let ret = fs::write(filepath, str_buf);
     CJSValue::val_bool(ret.is_ok())
   } else {
     let mut buf_size:usize = 0;
     let arr_buf = unsafe {q::JS_GetArrayBuffer(ctx,(&mut buf_size) as *mut usize,arg_slice[1]) };
     let buf = unsafe { std::slice::from_raw_parts(arr_buf, 4) };
     let ret = fs::write(filepath, buf);
     CJSValue::val_bool(ret.is_ok())
   }
}

pub extern "C" fn export_unlink(ctx: *mut q::JSContext, _: q::JSValue, _: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
   
   let path_str = unsafe { CJSValue::deserialize_value(*argv, ctx).unwrap().to_string() };
   let ret = fs::remove_file(path_str);
   CJSValue::val_bool(ret.is_ok())
}

pub extern "C" fn export_rmdir(ctx: *mut q::JSContext, _: q::JSValue, argc: ::std::os::raw::c_int, 
                              argv: *mut q::JSValue) -> q::JSValue {
   
   let path_str = unsafe { CJSValue::deserialize_value(*argv, ctx).unwrap().to_string() };
   let mut is_recursive = false;
   if argc > 1 {
      let arg_slice = unsafe { std::slice::from_raw_parts(argv, argc as usize) };
      let bval = CJSValue::deserialize_value(arg_slice[1], ctx).unwrap();
      is_recursive = bval.as_bool();
   };
   if is_recursive {
    let ret = fs::remove_dir_all(path_str);
    CJSValue::val_bool(ret.is_ok())
   } else {
    let ret = fs::remove_dir(path_str);
    CJSValue::val_bool(ret.is_ok())
   }
}