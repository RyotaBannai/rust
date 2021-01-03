pub mod nested;

pub fn function() {
  println!("Called my::function()")
}

#[allow(dead_code)]
fn list_dead_codes() {
  function();
}
