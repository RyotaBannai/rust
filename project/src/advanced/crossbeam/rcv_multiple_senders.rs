use owning_ref::StringRef;
use std::cell::RefCell;
use std::sync::{
  atomic::{AtomicBool, Ordering},
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

pub enum Messages {
  Count(u64),
  Lines(Vec<StringRef>),
}

// struct None();
// type C = dyn FnOnce() + Send;
#[derive(Debug)]
struct Executor {
  count: usize,
  handles: RefCell<Vec<Option<JoinHandle<()>>>>,
}
// pass fn as args ref: https://github.com/RyotaBannai/rust/blob/0c1daf7a96f38cf2406d32a94791498c5e8d1acd/project/src/archive/experiment/pass_fn_asarg.rs#L15

fn trim(s: String) -> StringRef {
  StringRef::new(s).map(str::trim)
}

fn lines_trimmed(input: &'_ mut dyn BufRead) -> Result<Vec<StringRef>, io::Error> {
  input.lines().map(|mb_line| mb_line.map(trim)).collect()
}

fn send_regularly(
  tx: Arc<Mutex<Sender<Messages>>>,
  close_thread: Arc<std::sync::atomic::AtomicBool>,
) {
  let sec = time::Duration::from_secs(3);
  let mut counter = 0;
  loop {
    if close_thread.load(Ordering::Relaxed) {
      println!("Closing thread! {:?}", thread::current().id());
      break;
    }
    counter += 1;
    thread::sleep(sec);
    tx.lock().unwrap().send(Messages::Count(counter));
  }
}

fn send_std_input(
  tx: Arc<Mutex<Sender<Messages>>>,
  close_thread: Arc<std::sync::atomic::AtomicBool>,
) {
  loop {
    if close_thread.load(Ordering::Relaxed) {
      println!("Closing thread! {:?}", thread::current().id());
      break;
    }
    let lines = match lines_trimmed(&mut stdin().lock()) {
      Ok(sr) => sr,
      Err(_) => panic!("failed to lines_trimmed"),
    };
    tx.lock().unwrap().send(Messages::Lines(lines));
  }
}

impl Executor {
  fn new() -> Self {
    Self {
      count: 0,
      handles: RefCell::new(vec![]),
    }
  }

  pub fn increment(&mut self) {
    self.count += 1
  }

  pub fn start(&mut self) {
    let (tx, rx) = mpsc::channel::<Messages>();
    let atx = Arc::new(Mutex::new(tx));
    let atx1 = atx.clone();
    let atx2 = atx.clone();
    let close_thread = Arc::new(AtomicBool::new(false));
    let close_thread1 = Arc::clone(&close_thread);
    let close_thread2 = Arc::clone(&close_thread);
    *self.handles.borrow_mut() = vec![
      Some(thread::spawn(move || send_regularly(atx1, close_thread1))),
      Some(thread::spawn(move || send_std_input(atx2, close_thread2))),
    ];

    loop {
      match rx.recv().unwrap() {
        Messages::Count(n) => println!("counting... {}", n),
        Messages::Lines(lines) => lines
          .iter()
          .for_each(|line| println!("  - {:?}", &**line as &str)),
      }
      self.increment();
      println!("> {}th call <", self.count);
      if self.count >= 10 {
        close_thread.swap(true, Ordering::Relaxed);
        break;
      }
    }

    //// block threads...
    // why Option?
    // -> solve this: move occurs because `*handle` has type `std::thread::JoinHandle<()>`, which does not implement the `Copy` trait rustc(E0507)
    // ref: https://stackoverflow.com/questions/57670145/how-to-store-joinhandle-of-a-thread-to-close-it-later
    // or stop with self itself, not reference.

    for handle in &mut *self.handles.borrow_mut() {
      (*handle)
        .take()
        .map(JoinHandle::join)
        .expect("Couldn't join my_thread on the main thread");
    }
  }
}

pub fn test() {
  let mut ex = Executor::new();
  ex.start();
}
