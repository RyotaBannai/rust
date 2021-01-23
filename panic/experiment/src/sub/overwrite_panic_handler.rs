use std::panic::{set_hook, take_hook};
use std::sync::{Arc, Mutex};

pub fn set_panic_handlers(handler_lists: Arc<Mutex<&Vec<&dyn Fn()>>>) {
  // もともと登録されていたハンドラを取り出す
  let hook_orig = take_hook();
  // hook: Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send>
  set_hook(Box::new(move |info| {
    // let lists = handler_lists.lock().unwrap();
    // for handler in lists.iter() {
    //   handler();
    // }
    hook_orig(info);
  }))
}
