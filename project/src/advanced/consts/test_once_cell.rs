use once_cell::sync::Lazy;

static LARGE_TEST: Lazy<String> = Lazy::new(|| load_large_text());

fn load_large_text() -> String {
  "So large text".to_string()
}

pub fn test() {
  println!("{}", *LARGE_TEST);
}
