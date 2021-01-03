/*
  RAII (Resource Acquisition is Initialization)
*/
pub struct Droppable;
impl Drop for Droppable {
  fn drop(&mut self) {
    println!("drop(): Resource will be released!")
  }
}

pub fn use_droppable() {
  {
    let _d = Droppable;
  }
  println!("use_droppable(): The Droppable should be released at the end of block.")
}
