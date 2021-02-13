use lazy_static::lazy_static;
use std::sync::Mutex;

// you can read global const from outside of files
// just by add pub before const
lazy_static! {
  pub static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<(), String> {
  let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
  db.push(fruit.to_string());
  Ok(())
}

pub fn demo() -> Result<(), String> {
  insert("apple")?;
  insert("orange")?;
  insert("peach")?;
  {
    let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.iter()
      .enumerate()
      .for_each(|(i, item)| println!("{}: {}", i, item));
  }
  insert("grape")?;
  Ok(())
}
