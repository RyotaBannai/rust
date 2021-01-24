use std::mem::{size_of, size_of_val};
use std::ops::Deref;
use std::str;

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

// check out https://doc.rust-lang.org/std/mem/fn.size_of.html
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
    // get bytes from memory and,
    // convert a slice of bytes to a string slice
    // https://doc.rust-lang.org/stable/std/str/fn.from_utf8.html
    let s = match str::from_utf8(std::slice::from_raw_parts(ptr, size_of_val(s))) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    dbg!(s); // you can convert string slice to String type by .to_owned()
  };
  dbg!(ptr.is_null()); // false
}

// x の中身をバイト列として見るための関数
fn as_raw_bytes<T: ?Sized>(x: &T) -> &[u8] {
  unsafe {
    //data: *const T, len: usize
    std::slice::from_raw_parts(x as *const T as *const u8, size_of_val(x))
  }
}

pub struct St<T: ?Sized> {
  pub x: u8,
  pub y: T,
}

pub fn check_byte_string() {
  let arr: [u16; 3] = [1, 2, 3];
  let arrref = &arr;
  let arrptr = &arr as *const [u16; 3]; // raw pointer
  let arrslice = &arr[..];
  let arrsliceptr = &arr[..] as *const [u16]; // u16 in array is defined at the beginning.

  let strslice = "multi chars";
  let clos = |x, y| x + y; // closure, arg type will be Fn(T, T) -> T
  let closref: &dyn Fn(u8, u8) -> u8 = &clos; // &Fn(u8, u8) -> u8 = &clos; is deprecated way to annotate an Object trait.
  let mut a = 0;
  let clos2 = |x: u8| {
    a += x;
    a
  };
  let clos2ref: &dyn FnMut(u8) -> u8 = &clos2;

  let sarr: Box<St<[u16; 3]>> = Box::new(St { x: 3, y: [1, 2, 3] });
  let sarrref = sarr.deref(); //
  let sslice: Box<St<[u16]>> = Box::new(St { x: 3, y: [1, 2, 3] });
  let ssliceref = sslice.deref();

  println!("arrref = {:?}", as_raw_bytes(arrref));
  println!("&arrref = {:?}", as_raw_bytes(&arrref));
  println!("&arrptr = {:?}", as_raw_bytes(&arrptr));
  println!("arrslice = {:?}", as_raw_bytes(arrslice));
  println!("&arrslice = {:?}", as_raw_bytes(&arrslice));
  println!("&arrsliceptr = {:?}", as_raw_bytes(&arrsliceptr));
  println!("strslice = {:?}", as_raw_bytes(strslice));
  println!("&strslice = {:?}", as_raw_bytes(&strslice));
  println!("closref = {:?}", as_raw_bytes(closref));
  println!("&closref = {:?}", as_raw_bytes(&closref));
  println!("clos2ref = {:?}", as_raw_bytes(clos2ref));
  println!("&clos2ref = {:?}", as_raw_bytes(&clos2ref));
  println!("sarrref = {:?}", as_raw_bytes(sarrref));
  println!("&sarrref = {:?}", as_raw_bytes(&sarrref));
  println!("ssliceref = {:?}", as_raw_bytes(ssliceref));
  println!("&ssliceref = {:?}", as_raw_bytes(&ssliceref));
}
