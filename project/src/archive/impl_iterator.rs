pub fn test_my_iter() {
  let it = Iter { current: 0, max: 5 };
  println!();
  for num in it {
    print!("{}, ", num)
  }
}

struct Iter {
  current: usize,
  max: usize,
}

impl Iterator for Iter {
  type Item = usize; // 出力する型の紐付け
  fn next(&mut self) -> Option<usize> {
    self.current += 1;
    if self.current - 1 < self.max {
      Some(self.current - 1)
    } else {
      None
    }
  }
}
