use std::sync::mpsc;
use std::thread;

pub fn msg_passing() {
  let mut handles = Vec::new();
  let mut snd_chan = Vec::new();
  let mut rcv_chan = Vec::new();
  let mut data = vec![1; 10];

  for x in 0..10 {
    // channel の戻り値は「送信・受信インスタンスのタプル」
    let (mtos_s, mtos_r) = mpsc::channel(); // main -> threads
    let (stom_s, stom_r) = mpsc::channel(); // threads -> main
    snd_chan.push(mtos_s); // main から send するインスタンスを格納
    rcv_chan.push(stom_r); // threads から値をもらうようの receive するインスタンスを格納

    handles.push(thread::spawn(move || {
      println!("{}th thread.. ", x);
      let mut data = mtos_r.recv().unwrap();
      data += 1;
      let _ = stom_s.send(data);
    }));
  }

  for x in 0..10 {
    // mtos_s: main -> each thread
    let _ = snd_chan[x].send(data[x]);
    // stom_r: each thread -> main
    data[x] = rcv_chan[x].recv().unwrap();
  }

  for handle in handles {
    let _ = handle.join();
  }
  dbg!(data);
}
