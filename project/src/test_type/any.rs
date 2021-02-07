use std::any::Any;
use std::fmt::Debug;

// https://doc.rust-lang.org/std/any/index.html
// pub trait Any: 'static
fn log<T: Any + Debug>(value: &T) {
  let value_any = value as &dyn Any;

  match value_any.downcast_ref::<String>() {
    Some(as_string) => {
      println!("String ({}): {}", as_string.len(), as_string);
    }
    None => {
      println!("{:?}", value);
    }
  }

  // dbg!(value_any.is::<String>());
}

fn do_work<T: Any + Debug>(value: &T) {
  log(value);
  // do something
}

// https://stackoverflow.com/questions/55300053/what-does-mean-for-rust-slices-and-what-is-it-called
// #![feature(core_intrinsics)]

// fn print_type_of<T>(_: &T) {
//     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
// }

pub fn test() {
  let my_string = "Hello World".to_string();
  do_work(&my_string);

  let my_i8 = 100i8;
  do_work(&my_i8);
}
