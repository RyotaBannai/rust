struct S {
  x: u32,
}

pub fn test() {
  // address will change after move.
  let s = S { x: 0 };
  println!("Before move {:?}", &s as *const _);
  let ss = s;
  println!("After move {:?}", &ss as *const _);
}
