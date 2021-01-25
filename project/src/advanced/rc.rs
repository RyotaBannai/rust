// Creating a recursive data structure:
#[derive(Debug)]
enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}
use List::{Cons, Nil};

pub fn test_rc() {
  let a = Cons(5, Box::new(List::Cons(10, Box::new(Nil))));
  let b = Cons(3, Box::new(a));
  println!("{:?}", b);
  // Cons(3, Cons(5, Cons(10, Nil)))
  // let c = Cons(4, Box::new(a)); // Error: b がすでに所有権を保持しているためエラーになる
  // println!("{:?}", a);          // Error: 上記と同じ理由
}
