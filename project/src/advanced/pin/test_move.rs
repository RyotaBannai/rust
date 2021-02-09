// use pin_utils::pin_mut;
use std::marker::PhantomPinned;
use std::pin::Pin;

struct S {
  x: u32,
  _pinned: PhantomPinned,
}

impl S {
  pub fn new(x: u32) -> Self {
    Self {
      x: x,
      _pinned: PhantomPinned,
    }
  }
}

pub fn test() {
  // address will change after move.
  let s = S::new(0);
  println!("Before move {:?}", &s.x as *const _);
  let ss = s;
  println!("After move {:?}", &ss.x as *const _);
}

// The address of struct itself will change.
pub fn test_not_move() {
  // address will change after move.
  let s = S::new(0);
  let obj: Pin<Box<S>> = Box::pin(s);
  // pin_mut!(s);
  println!("Before move {:?}", &obj.x as *const _);
  let ss = obj;
  println!("After move {:?}", &ss.x as *const _);
}
