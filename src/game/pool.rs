pub struct Pool<T> {
 new_fn  :fn() -> T,
 reset_fn:fn(T)
}

impl<T> Pool<T> {
    pub fn create(new:fn() -> T,reset:fn(T)) -> Self {
        Pool {
            new_fn:new,
            reset_fn:reset
        }
    }

    pub fn use_value(&self) -> T {
      let new_func = self.new_fn;
      new_func()
    } 
}