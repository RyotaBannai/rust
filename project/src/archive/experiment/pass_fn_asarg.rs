// Fn(&Vec<i32>) -> f64 is not a sized type, and cannot be passed by value.
// You can only pass:
// trait objects (references or boxes), or
// values of (sized) types implementing a trait.
fn stat_query<F>(dataset: &Vec<Vec<i32>>, query: F) -> f64
where
  F: Fn(&Vec<i32>) -> f64,
{
  if dataset.len() == 0 {
    0.
  } else {
    dataset.iter().map(|ref fv| query(&fv)).sum::<f64>() / (dataset.len() as f64)
  }
}
// pass fn as args ref: https://stackoverflow.com/questions/36390665/how-do-you-pass-a-rust-function-as-a-parameter
// or you can do like
// fn fun_test_impl(value: i32, f: impl Fn(i32) -> i32) -> i32 {}
// fn fun_test_dyn(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {}
// fn fun_test_ptr(value: i32, f: fn(i32) -> i32) -> i32 {}

pub fn main() {
  let fv1: Vec<i32> = vec![1, 1, 1, 1, 1];
  let fv2: Vec<i32> = vec![1, 0, 1, 0, 1];
  let my_dataset = vec![fv1, fv2];
  fn my_query(ref fv: &Vec<i32>, threshold: i32) -> f64 {
    if fv.iter().sum::<i32>() > threshold {
      1.
    } else {
      0.
    }
  }
  println!("{}", stat_query(&my_dataset, |ref fv| my_query(fv, 3)));
}
// ref https://stackoverflow.com/questions/47295884/passing-a-closure-to-a-function-expecting-a-stdopsfn
