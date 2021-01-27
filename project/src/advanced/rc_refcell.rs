// ref https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#keeping-track-of-borrows-at-runtime-with-refcellt
#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}
use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

pub fn test() {
  let value = Rc::new(RefCell::new(5));
  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *value.borrow_mut() += 10; // add 10 and reflect the change in all owners
  dbg!(a);
  dbg!(b);
  dbg!(c);
}

// The standard library has other types that provide interior mutability, such as
// ・Cell<T>, which is similar except that instead of giving references to the inner value, the value is copied in and out of the Cell<T>.
// ・Mutex<T>, which offers interior mutability that’s safe to use across threads
