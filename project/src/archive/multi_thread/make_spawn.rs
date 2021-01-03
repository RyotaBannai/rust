use std::thread;

pub fn make_spawn() {
  let mut handles = Vec::new();
  for x in 0..10 {
    handles.push(thread::spawn(move || {
      println!("hello world{}", x);
    }));
  }

  for handle in handles {
    let _ = handle.join();
  }
}
