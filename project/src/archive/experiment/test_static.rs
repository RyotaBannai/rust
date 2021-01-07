static NUM: i32 = 32;
const ROOT: &str = "Rust";

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

// Reference: https://laysakura.github.io/2020/05/21/rust-static-lifetime-and-static-bounds/
fn i_need_static_bound_type<T: 'static>(v: T) {}

// 参照を含まない
struct IHaveValue(String);

// 'static ライフタイムの参照だけを含む
struct IHaveStaticRef(&'static str);

// 'a というライフタイムの参照だけを含む
struct IHaveNonStaticRef<'a>(&'a str);

fn test_lifetime() {
  i_need_static_bound_type(IHaveValue("abc".to_string())); // &str -> String
  i_need_static_bound_type(IHaveStaticRef("abc"));
  i_need_static_bound_type(IHaveNonStaticRef("abc"));

  // 関数のスコープという 'static よりも短い lifetime の`参照`を渡しているので以下のパターンはエラー
  // let local_string: String = format!("abc");
  // i_need_static_bound_type(IHaveNonStaticRef(&local_string));
}

async fn some_great_afn() {
  let local_string: String = "abc".to_string();
  some_great_afn_with_borrowing(local_string).await;
}

async fn some_great_afn_with_borrowing(x: String) {}

fn some_great_fn() {
  let local_string: String = "abc".to_string();
  some_great_afn_with_borrowing(local_string);
}

fn some_great_fn_with_borrowing(x: String) {}
