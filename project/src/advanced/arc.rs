use std::sync::{Arc, Mutex};
use std::thread::spawn;

pub fn share_data_from_multi_threads() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];
  for _ in 0..10 {
    // value moved into closure here, in previous iteration of loop
    // Arc<T> = Rc<T> + (スレッドセーフ ＋ 内部可変性) (Atomic Rc)
    let shared_ownership = Arc::clone(&counter);
    let handle = spawn(move || {
      let mut num = shared_ownership.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    // wait all handlers.
    handle.join().unwrap();
  }
  println!("{:?}", counter);
}
