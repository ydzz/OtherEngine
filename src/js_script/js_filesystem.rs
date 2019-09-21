use quick_js::{Context};

pub struct JSFileSystemModule {
   a:[i32;5]
}

impl crate::game_core::ScriptModule<Context> for JSFileSystemModule {
   fn new() -> JSFileSystemModule {
      JSFileSystemModule { a : [1,2,3,4,5] }
   }

   fn attach_script_context(&self,ctx:Context) {
      let a = [1,2,3,4,5,6];
      //ctx.add_callback(name: &str, callback: impl Callback<F> + 'static)
   }
}