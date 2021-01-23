pub mod my_drop;
pub mod overwrite_panic_handler;
use std::sync::{Arc, Mutex};

pub fn use_droppable() {
  let mut my_droppable = my_drop::Droppable { name: "dropper" };
  assert!(false); // cause a panic
}

pub fn do_over_write_panic_handler() {
  let handlers = Arc::new(Mutex::new(&vec![&|| println!("panic handler 1")]));
  // overwrite_panic_handler::set_panic_handlers(handlers)
}
