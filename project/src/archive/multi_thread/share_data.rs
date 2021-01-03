use std::sync::{Arc, Mutex};
use std::thread;

pub fn share_data() {
  let mut handles = Vec::new();
  let data = Arc::new(Mutex::new(vec![1; 10]));

  for x in 0..10 {
    let data_ref = data.clone(); // 各スレッドに所有権を生成 -> 「参照カウンタ」が増える
    handles.push(thread::spawn(move || {
      println!("{}th thread do culc..", x);

      let mut data = data_ref.lock().unwrap();
      data[x] += 1;
    }));
  }

  for handle in handles {
    let _ = handle.join();
  }

  dbg!(data);
}
