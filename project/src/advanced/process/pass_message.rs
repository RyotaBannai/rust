extern crate nix;
extern crate tempfile;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use nix::sys::stat;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, mkfifo, ForkResult, Pid};

use tempfile::tempdir;

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
      child
    }
    Ok(ForkResult::Child) => {
      println!("Main({}) forked a PPID ({})", getpid(), getppid());
      sleep(Duration::from_secs(3));
      let mut file = OpenOptions::new().read(true).open(&fifo_path).unwrap();
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap(); // use ? instead of unwrap()
      println!("{}", contents);
      exit(0);
    }
    Err(_) => panic!("Fork failed"), // return を期待している時は　panic にする
  };
  {
    let mut file = OpenOptions::new().write(true).open(&fifo_path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
  }

  match waitpid(child_pid, None) {
    Ok(status) => println!("Child exited({:?})", status),
    Err(_) => println!("waitpid() failed"),
  }
}
