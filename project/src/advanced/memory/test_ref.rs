use std::cell::{Ref, RefCell, RefMut};

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
