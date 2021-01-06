use std::fmt::Debug; // Trait to bound with

/**
 * Derive
 * https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
 *
 * // The `derive` attribute automatically creates the implementation
 * // required to make this `struct` printable with `fmt::Debug`
 */

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `T` is bounded such that any
// *references* in `T` must outlive `'a`.
// Additionally, the lifetime of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T)
where
  T: Debug,
{
  println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T)
where
  T: Debug + 'a,
{
  println!("`print_ref`: t is {:?}", t);
}

fn test_bounds() {
  let x = 7;
  let ref_x = Ref(&a);
  print_ref(&ref_x);
  print(ref_x);
}
