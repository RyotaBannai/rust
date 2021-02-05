use std::mem::{self, MaybeUninit};

/**  
 *  The compiler then knows to not make any incorrect assumptions or optimizations on this code.
 * You can think of MaybeUninit<T> as being a bit like Option<T> but without any of the run-time tracking and without any of the safety checks.
*/

pub fn basics() {
  // below code are same.
  // let x: &i32 = unsafe { mem::zeroed() }; // undefined behavior!
  // let y: &i32 = unsafe { MaybeUninit::zeroed().assume_init() }; // undefined behavior!

  // Create an explicitly uninitialized reference.
  // The compiler knows that data inside a `MaybeUninit<T>` may be invalid,
  // and hence this is not UB:
  let mut x = MaybeUninit::<&i32>::uninit();
  // Set it to a valid value.
  unsafe {
    x.as_mut_ptr().write(&0);
  }
  // Extract the initialized data -- this is only allowed *after* properly
  // initializing `x`!
  let x = unsafe { x.assume_init() };
  dbg!(x); // x = 0
}

pub fn make_vec(out: *mut Vec<i32>) {
  // `write` does not drop the cold contents, which is important.
  unsafe {
    out.write(vec![1, 2, 3]);
  }
}

pub fn out_pointer() {
  let mut v = MaybeUninit::uninit();

  make_vec(v.as_mut_ptr());

  let v = unsafe { v.assume_init() };
  assert_eq!(&v, &[1, 2, 3]);
  dbg!(&v);
}

pub fn normal_out_pointer() {
  let mut v = Vec::new();
  make_vec(&mut v);
  dbg!(v);
}

pub fn test() {
  out_pointer();
  normal_out_pointer();
}
