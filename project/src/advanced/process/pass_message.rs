extern crate nix;
extern crate tempfile;

use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::stat;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, mkfifo, ForkResult, Pid};

use tempfile::tempdir;
use unicode_segmentation::UnicodeSegmentation;

// tokio flush to close file https://github.com/tokio-rs/tokio/issues/2307
// 読み出し用にオープンすると他のプロセスによって書き込み用にオープンされるまでブロックされる。逆も同様。
pub fn test() {
  let tmp_dir = tempdir().unwrap();
  let fifo_path = tmp_dir.path().join("tmp.pipe");

  // Creates new fifo special file (named pipe) with path path and access rights mode.
  match mkfifo(&fifo_path, stat::Mode::S_IRWXU) {
    Ok(_) => println!("created {:?}", fifo_path),
    Err(err) => println!("Error creating fifo: {}", err),
  }

  let child_pid = match unsafe { fork() } {
    Ok(ForkResult::Parent { child, .. }) => {
      println!("Main({}) forked a child({})", getpid(), child);
      parent_process(&fifo_path);
      child
    }
    Ok(ForkResult::Child) => {
      println!("Main({}) forked a PPID ({})", getpid(), getppid());
      child_process(&fifo_path);
      exit(0);
    }
    Err(_) => panic!("Fork failed"),
  };

  // child が最後まで read するまで wait
  match waitpid(child_pid, None) {
    Ok(status) => println!("Child exited({:?})", status),
    Err(_) => println!("waitpid() failed"),
  }
}

// string length https://stackoverflow.com/questions/46290655/get-the-string-length-in-characters-in-rust
// https://users.rust-lang.org/t/quotient-and-remainder/16093
fn child_process(fifo_path: &PathBuf) {
  loop {
    let mut file = OpenOptions::new().read(true).open(&fifo_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // use ? instead of unwrap()
    println!("{}", contents);
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: Empty }'
    let base: i32 = 10;
    let dev_rem = div_rem(
      contents.parse::<i32>().unwrap(),
      // base.pow(contents.graphemes(true).count().try_into().unwrap()),
      base,
    );
    println!("{:?}", dev_rem);
    if dev_rem.1 == 5 {
      println!("OK, that's enough too");
      break;
    }
  }
}
// convert bytes between String https://gist.github.com/RyotaBannai/4c99573d86a3ef2000d3681ad2c7c264
// child が read のロックをする前に parent が複数回 open して複数回追記してしまうことがある
fn parent_process(fifo_path: &PathBuf) {
  let mut count = 0u32;
  loop {
    count += 1;
    // block ({})で lifetime をきるか、drop するかで file を close する
    let mut file = OpenOptions::new().write(true).open(&fifo_path).unwrap();
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }'
    file
      .write_all(&format!("{}", count).as_bytes().to_vec())
      .unwrap();
    // drop(file);
    // when pass through this loop block, file closes by itself. no need drop() in this case.
    if count == 5 {
      println!("OK, that's enough");
      break; // exit this loop
    }
  }
}

pub fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(
  x: T,
  y: T,
) -> (T, T) {
  let quot = x / y;
  let rem = x % y;
  (quot, rem)
}

pub fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
  div_rem(x, y)
}
