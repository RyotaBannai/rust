static NUM: i32 = 32;
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
  &NUM
}

pub fn main_fn() {
  // (1)
  {
    let static_string = "string";
    println!("{}", static_string);
  }
  // (2)
  {
    let lifetime_num = 9;
    let coerced_static = coerce_static(&lifetime_num);
    println!("coerced_static: {}", coerced_static);
  }
  println!("NUM: {} stays accessible!", NUM); // &NUM ok as well.
}

use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
  println!("'static value passed in is: {:?}", input);
}

fn use_it() {
  // i is owned and contains no references, thus it's 'static:
  let i = 5;
  print_it(i);

  // oops, &i only has the lifetime defined by the scope of
  // use_it(), so it's not 'static:
  // print_it(&i); // >>> error
}
