use std::cell::{Ref, RefCell, RefMut};

pub fn test() {
  let age: RefCell<u32> = RefCell::new(30);
  {
    let age_ref: Ref<u32> = age.borrow();
    println!("{}", *age_ref);
  }

  let mut age_mut_ref: RefMut<u32> = age.borrow_mut(); // do not forget mut even if you use RefMut
  *age_mut_ref += 1;

  println!("{}", *age_mut_ref);
}
