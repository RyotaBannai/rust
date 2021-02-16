use std::cell::{Ref, RefCell, RefMut};

// ref https://www.fpcomplete.com/blog/of-course-it-compiles-right/
// you can do few operations on RefCell and get Ref/RefMut value.
// https://doc.rust-lang.org/nightly/core/cell/struct.Ref.html

pub fn test() {
  let age: RefCell<u32> = RefCell::new(30);
  {
    let age_ref: Ref<u32> = age.borrow();
    println!("{}", *age_ref);

    // or you can drop manually
    // std::mem::drop(age_ref);
  }

  let mut age_mut_ref: RefMut<u32> = age.borrow_mut(); // do not forget mut even if you use RefMut
  *age_mut_ref += 1;
  println!("{}", *age_mut_ref);

  let mut age_mut_reference: &u32 = &mut age_mut_ref;
  println!("{}", *age_mut_reference);

  // inner block
  // you can't return reference, bc when goes out of block, the age_ref will be dropped.
  // let age_reference: &u32 = {
  //   let age_ref: Ref<u32> = age.borrow();
  //   &age_ref
  //   // but returning &age.borrow() will work as known as temporary lifetime extension
  //   // https://doc.rust-lang.org/reference/destructors.html?highlight=temporary,life#temporary-lifetime-extension
  // };
}

// this errors because, hello has a reference to all_tags but in the world block
// this tries to burrow as mut. 'when you burrow as mut reference, the reference is only one reference.'
pub fn demo() {
  let all_tags: RefCell<Vec<String>> = RefCell::new(Vec::new());
  println!("Adding hello");
  let hello: &str = {
    all_tags.borrow_mut().push("Hello".to_string());
    &all_tags.borrow()[0]
  };
  println!("Adding world");
  let world: &str = {
    all_tags.borrow_mut().push("World".to_string());
    &all_tags.borrow()[0]
  };
  println!("{} {}", hello, world)
}
