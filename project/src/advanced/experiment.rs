use std::mem::{size_of, size_of_val};

// https://stackoverflow.com/questions/25754863/how-to-create-a-rust-struct-with-string-members/
struct S<'a> {
  //  if the lifetime of the string is unknown use 'a
  fullname: &'a str,
  // static lifetime i.e. the lifetime of the program.
  // fullname: &'static str
  // if the strnig has to be owned by the struct use String
  // lastname: String

  // and, if you're not sure if the string will be owned or not, then use borrow::Cow
  // https://doc.rust-lang.org/std/borrow/enum.Cow.html
}

pub fn check_byte_size() {
  let size = size_of::<i32>();
  let s_size = size_of::<S>();
  println!("{}", size);
  println!("{}", s_size);
}

// https://doc.rust-lang.org/std/mem/fn.size_of.html
pub fn test_all_pointer_size_is_the_same() {
  // all of them are 8
  dbg!(size_of::<*const i16>());
  dbg!(size_of::<*const i32>());

  dbg!(size_of::<Box<i8>>());
  dbg!(size_of::<Option<&i32>>());
  dbg!(size_of::<Box<Option<&i32>>>());
}

// https://doc.rust-lang.org/std/primitive.pointer.html
pub fn get_pointer() {
  let s: &str = "Follow the rabbit";
  let ptr: *const u8 = s.as_ptr(); // get actual pointer
  dbg!(ptr);
  unsafe {
    // dbg!(*ptr.offset(1) as char);
    if let Some(val_back) = ptr.as_ref() {
      dbg!(val_back); // can get only the first address
    }
  };
  dbg!(ptr.is_null()); // false
}
