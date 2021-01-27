use std::cell::RefCell;
use std::rc::{Rc, Weak};

// ref https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent

// Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist.
// This is a case for weak references!

#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  // A node will be able to refer to its parent node but doesn’t own its parent. Weak!
  children: RefCell<Vec<Rc<Node>>>,
}

pub fn test() {
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  // you must make sure the value still exists. Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>. You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped.
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None

  let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  // The lack of infinite output indicates that this code didn’t create a reference cycle!!
  // We can also tell this by looking at the values we get from calling Rc::strong_count and Rc::weak_count.
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
