extern crate nix;
extern crate tempfile;

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

  match unsafe { fork() } {
    Ok(ForkResult::Parent { child, .. }) => {
      println!("Main({}) forked a child({})", getpid(), child);
      parent_process(&fifo_path);
    }
    Ok(ForkResult::Child) => {
      println!("Main({}) forked a PPID ({})", getpid(), getppid());
      child_process(&fifo_path);
      exit(0);
    }
    Err(_) => panic!("Fork failed"),
  };
}

fn child_process(fifo_path: &PathBuf) {
  loop {
    let mut file = OpenOptions::new().read(true).open(&fifo_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // use ? instead of unwrap()
    println!("{}", contents);
    if contents.parse::<i32>().unwrap() == 5 {
      println!("OK, that's enough too");
      break;
    }
  }
}
// convert bytes between String https://gist.github.com/RyotaBannai/4c99573d86a3ef2000d3681ad2c7c264
fn parent_process(fifo_path: &PathBuf) {
  let mut count = 0u32;
  loop {
    count += 1;
    // block ({})で lifetime をきるか、drop するかで file を close する
    let mut file = OpenOptions::new().write(true).open(&fifo_path).unwrap();
    file
      .write_all(&format!("{}", count).as_bytes().to_vec())
      .unwrap();
    drop(file);
    if count == 5 {
      println!("OK, that's enough");
      break; // exit this loop
    }
  }
}
