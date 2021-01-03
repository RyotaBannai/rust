pub trait Tweet {
  fn tweet(&self);
  fn tweet_twice(&self) {
    self.tweet();
    self.tweet();
  }
  fn shout(&self) {
    println!("Yeahh!")
  }
}

pub struct Dove;
pub struct Duck;

impl Tweet for Dove {
  fn tweet(&self) {
    println!("Dove: Coo!")
  }
}
impl Tweet for Duck {
  fn tweet(&self) {
    println!("Duck: Quack!")
  }
}
