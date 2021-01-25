// ref https://doc.rust-lang.org/book/ch15-04-rc.html
// Creating a recursive data structure:
#[derive(Debug)]
enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}
use List::{Cons, Nil};

pub fn test_box() {
  let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
  let b = Cons(3, Box::new(a));
  println!("{:?}", b);
  // Cons(3, Cons(5, Cons(10, Nil)))
  // let c = Cons(4, Box::new(a)); // Error: b がすでに所有権を保持しているためエラーになる
  // println!("{:?}", a);          // Error: 上記と同じ理由
}

#[derive(Debug)]
enum ListRc<T> {
  ConsRc(T, Rc<ListRc<T>>),
  NilRc,
}

use std::rc::Rc;
use ListRc::{ConsRc, NilRc};

pub fn test_rc() {
  let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
  let b = ConsRc(3, Rc::clone(&a));
  let c = Rc::new(ConsRc(4, Rc::clone(&a)));
  let d = ConsRc(10, Rc::clone(&c));
  println!("{}", Rc::strong_count(&a)); // reference count is 3 at this point.
  println!("{:?}", Rc::clone(&a));
  println!("{:?}", a);
  println!("{:?}", a);
  // no errors
  // The call to Rc::clone only increments the reference count, which doesn’t take much time
}
