use std::process::Command;
use std::str;

// spawn() Executes the command as a child process, returning a handle to it.
// By deafult, stdin, stdout and stderr are inherited from the parent.

pub fn test() {
  let output = if cfg!(taret_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .output()
      .expect("failed to excecute process")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("failed to excecute process")
  };

  // let hello =
  match str::from_utf8(&output.stdout) {
    // without giving a pointer...
    // expected `&[u8]`, found struct `std::vec::Vec`
    Ok(hello) => println!("{:?}", &hello),
    Err(e) => panic!("INvalid UTF-8 sequence: {}", e),
  }
  // or let s = String::from_utf8(&output.stdout).expect("Found invalid UTF-8");
  // you don't need an error handling with from_utf8_lossy e.g.
  let s = String::from_utf8_lossy(&output.stdout);
  println!("{:?}", s);
}
