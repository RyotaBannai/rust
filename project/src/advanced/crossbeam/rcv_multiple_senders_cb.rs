use crossbeam;
use crossbeam::thread::Scope;
// use crossbeam_channel::{unbounded, Receiver, Sender};

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

// http://steavevaivai.hatenablog.com/entry/2020/08/09/132038

pub enum Messages {
  Count(u64),
  Lines(Vec<StringRef>),
}

#[derive(Debug)]
struct Executor {
  count: usize,
  handles: RefCell<Vec<Option<JoinHandle<()>>>>,
}

fn trim(s: String) -> StringRef {
  StringRef::new(s).map(str::trim)
}

fn lines_trimmed(input: &'_ mut dyn BufRead) -> Result<Vec<StringRef>, io::Error> {
  input.lines().map(|mb_line| mb_line.map(trim)).collect()
}

fn send_regularly(tx: Sender<Messages>, scope: &Scope<'_>) {
  scope.spawn(move |s| {
    let sec = time::Duration::from_secs(3);
    let mut counter = 0;
    loop {
      counter += 1;
      thread::sleep(sec);
      tx.send(Messages::Count(counter));
    }
  });
}

fn send_std_input(tx: Sender<Messages>, scope: &crossbeam::thread::Scope<'_>) -> () {
  scope.spawn(move |s| loop {
    let lines = match lines_trimmed(&mut stdin().lock()) {
      Ok(sr) => sr,
      Err(_) => panic!("failed to lines_trimmed"),
    };
    tx.send(Messages::Lines(lines));
  });
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

  pub fn start<'a>(&mut self) -> Result<(), ()> {
    crossbeam::scope(|s| {
      let (tx, rx) = mpsc::channel();
      // let (tx, rx) = unbounded();
      let txc = mpsc::Sender::clone(&tx);
      send_regularly(tx, s);
      send_std_input(txc, s);

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
          break;
        }
      }
    })
    .unwrap();
    Ok(())
  }
}

pub fn test() {
  let mut ex = Executor::new();
  ex.start();
}
