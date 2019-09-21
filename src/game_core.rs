pub trait ScriptModule <T> {
 fn new() -> Self;
 fn attach_script_context(&self,ctx:T);
}