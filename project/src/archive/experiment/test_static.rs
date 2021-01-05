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

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
  first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
  first
}

fn main() {
  let first = 2; // Longer lifetime
  {
    let second = 3; // Shorter lifetime

    println!("The product is {}", multiply(&first, &second));
    println!("{} is the first", choose_first(&first, &second));
  };
}
