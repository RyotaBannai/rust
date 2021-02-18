struct AverageCollection {
  list: Vec<i32>,
  average: f64,
}

impl AverageCollection {
  fn new() -> Self {
    AverageCollection {
      list: Vec::new(),
      average: 0.0,
    }
  }

  fn push(&mut self, new_num: i32) {
    self.list.push(new_num);
    self.update_average();
  }

  fn remove(&mut self, nth: usize) -> i32 {
    self.list.remove(nth)
  }

  fn remove_first_value(&mut self, value: i32) -> i32 {
    self.list.remove(
      self
        .list
        .iter()
        .position(|x| *x == value)
        .expect("this value not found"),
    )
  }

  fn remove_all_values(&mut self, value: i32) {
    self.list.retain(|x| *x != value);
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.iter().len() as f64;
  }

  fn get_averate(&mut self) -> f64 {
    self.average
  }
}

pub fn test() {
  let mut ac = AverageCollection::new();
  ac.push(1);
  ac.push(2);
  ac.push(3);
  println!("{}", ac.get_averate());
}
