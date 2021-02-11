use owning_ref::StringRef;
use std::io::{self, stdin, BufRead};

fn read_std_input() -> Result<String, io::Error> {
  println!("enter text: ");
  let mut s = String::new();
  stdin().read_line(&mut s)?;
  Ok((&s).trim_end().to_string()) // remove \n at the end.
}

// of course you can return String instead of StringRef for simplicity
// but with owning_ref you can move it around w/o reallocating the address,
// which means you don't lose the reference to it and never break things.
fn trim(s: String) -> StringRef {
  StringRef::new(s).map(str::trim)
}

fn lines_trimmed(input: &'_ mut dyn BufRead) -> Result<Vec<StringRef>, io::Error> {
  input.lines().map(|mb_line| mb_line.map(trim)).collect() // doesn't reallocate when returning from map
}

pub fn test() -> Result<(), io::Error> {
  // let result = match read_std_input() {
  //   Ok(s) => {
  //     println!("you entered: '{}'", s);
  //     s
  //   }
  //   Err(e) => panic!("panic!!"),
  // };

  let lines = lines_trimmed(&mut stdin().lock())?;

  println!("Trimmed lines:");
  lines
    .iter()
    .for_each(|line| println!("  - {:?}", &**line as &str));
  Ok(())
}
