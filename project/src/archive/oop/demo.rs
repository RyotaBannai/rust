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

  fn push(&mut self, new_num: i32) -> Vec<i32> {
    self.list.push(new_num);
    self.update_average();
    self.list.clone()
  }

  fn push_all(&mut self, another_vec: &mut Vec<i32>) -> Vec<i32> {
    self.list.append(another_vec);
    self.update_average();
    self.list.clone()
  }

  fn remove(&mut self, nth: usize) -> i32 {
    let removed_item = self.list.remove(nth);
    self.update_average();
    removed_item
  }

  fn remove_first_value(&mut self, value: i32) -> i32 {
    let removed_item = self.list.remove(
      self
        .list
        .iter()
        .position(|x| *x == value)
        .expect("this value not found"),
    );
    self.update_average();
    removed_item
  }

  fn remove_all_values(&mut self, value: i32) {
    self.list.retain(|x| *x != value);
    self.update_average();
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.iter().len() as f64;
  }

  fn get_average(&mut self) -> f64 {
    self.average
  }
}

pub fn test() {
  let mut ac = AverageCollection::new();
  ac.push(1);
  ac.push(2);
  ac.push(3);
  println!("{}", ac.get_average());

  ac.push_all(&mut vec![4, 2, 3]);
  println!("{}", ac.get_average());

  ac.remove_all_values(2);
  println!("{}", ac.get_average());
}
