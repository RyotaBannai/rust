pub mod my_drop;

pub fn use_droppable() {
  let mut my_droppable = my_drop::Droppable { name: "dropper" };
  assert!(false); // cause a panic
}
