pub struct Droppable {
  pub name: &'static str,
}

pub struct A(bool);

impl Drop for Droppable {
  fn drop(&mut self) {
    println!("> Dropping {}", self.name)
  }
}

impl Drop for A {
  fn drop(&mut self) {
    if self.0 {
      eprintln!("Something happened! Cleaning up...")
    }
  }
}
