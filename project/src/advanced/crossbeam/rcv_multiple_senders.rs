use owning_ref::StringRef;
use std::cell::RefCell;
use std::sync::{
  atomic::Ordering,
  mpsc::{self, SendError, Sender},
  Arc, Mutex,
};
use std::{
  io::{self, stdin, BufRead},
  thread::{self, JoinHandle},
  time,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
  #[error(transparent)]
  SendNumError(#[from] SendError<u64>),
  #[error(transparent)]
  SendSRError(#[from] SendError<Vec<StringRef>>),
}

enum Messages {
  Count(u64),
  Lines(Vec<StringRef>),
}

// struct None();
// type C = dyn FnOnce() + Send;
#[derive(Debug)]
struct Executor {
  count: usize,
  handles: Arc<Mutex<RefCell<Vec<Option<JoinHandle<()>>>>>>,
}
// pass fn as args ref: https://github.com/RyotaBannai/rust/blob/0c1daf7a96f38cf2406d32a94791498c5e8d1acd/project/src/archive/experiment/pass_fn_asarg.rs#L15

fn trim(s: String) -> StringRef {
  StringRef::new(s).map(str::trim)
}

fn lines_trimmed(input: &'_ mut dyn BufRead) -> Result<Vec<StringRef>, io::Error> {
  input.lines().map(|mb_line| mb_line.map(trim)).collect()
}

impl Executor {
  fn new() -> Self {
    Self {
      count: 0,
      handles: Arc::new(Mutex::new(RefCell::new(vec![]))),
    }
  }

  pub fn increment(&mut self) {
    self.count += 1
  }

  pub fn start(&mut self) {
    let (tx, rx) = mpsc::channel::<Messages>();
    let atx = Arc::new(Mutex::new(tx));
    fn send_regularly(tx: Arc<Mutex<Sender<Messages>>>) {
      let sec = time::Duration::from_secs(3);
      let mut counter = 0;
      loop {
        counter += 1;
        thread::sleep(sec);
        tx.lock()
          .unwrap()
          .send(Messages::Count(counter * sec.as_secs()));
      }
    };

    fn send_std_input(tx: Arc<Mutex<Sender<Messages>>>) {
      loop {
        let lines = match lines_trimmed(&mut stdin().lock()) {
          Ok(sr) => sr,
          Err(_) => panic!("failed to lines_trimmed"),
        };
        tx.lock().unwrap().send(Messages::Lines(lines));
      }
    };

    let atx1 = atx.clone();
    let atx2 = atx.clone();
    let wait_handlers = self.handles.clone();
    wait_handlers.lock().unwrap().replace_with(|_| {
      vec![
        Some(thread::spawn(move || send_regularly(atx1))),
        Some(thread::spawn(move || send_std_input(atx2))),
      ]
    });
    //// block threads...
    // why Option?
    // -> solve this: move occurs because `*handle` has type `std::thread::JoinHandle<()>`, which does not implement the `Copy` traitrustc(E0507)
    // ref: https://stackoverflow.com/questions/57670145/how-to-store-joinhandle-of-a-thread-to-close-it-later
    // or stop with self itself, not reference.

    // for handle in &mut *wait_handlers.lock().unwrap().borrow_mut() {
    //   // (*handle).swap(true, Ordering::Relaxed);
    //   (*handle)
    //     .take()
    //     .map(JoinHandle::join)
    //     .expect("Couldn't join my_thread on the main thread");
    // }

    loop {
      match rx.recv().unwrap() {
        Messages::Count(n) => println!("counting... {}", n),
        Messages::Lines(lines) => lines
          .iter()
          .for_each(|line| println!("  - {:?}", &**line as &str)),
      }
      self.increment();
      println!("> {}th call <", self.count);
    }
  }
}

pub fn test() {
  let mut ex = Executor::new();
  ex.start();
}
