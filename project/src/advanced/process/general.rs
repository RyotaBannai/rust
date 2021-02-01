extern crate nix;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult, Pid};
use std::process::exit;

// ref https://gkuga.hatenablog.com/entry/2020/01/12/185917
fn show_pid() {
  println!("PID: {}", getpid()); // pid
  println!("PID: {}", Pid::this()); // pid

  // pid of the parent of calling process
  println!("PID: {}", getppid());
}

/**
 * Fork:
 * Create a new child process duplicating the parent process
 * After calling the fork system call (successfully) two processes will be created that are
 * identical with the exception of their pid and the return value of this function.
 */

fn naive() {
  println!("(Main({}), PPID({}))", getppid(), getpid()); // (Main(6765, PPID(8359))
  match unsafe { fork() } {
    Ok(ForkResult::Parent { child, .. }) => {
      // parent branch
      println!("Main({}) forked a child({})", getpid(), child);
    }
    Ok(ForkResult::Child) => {
      // parent child
      println!("Main({}) forked a PPID ({})", getpid(), getppid());
      // ppid() が 1 になるのは、親プロセスが先に終了してしまい、
      // PID が １ の init プロセスに付け替えれるため
      // 親プロセスを子プロセス終了まで待つようにする → waitpid()
    }
    Err(_) => println!("Fork failed"),
  }
}

fn wait_child() {
  println!("(Main({}), PPID({}))", getppid(), getpid());
  let child_pid = match unsafe { fork() } {
    Ok(ForkResult::Parent { child, .. }) => {
      println!("Main({}) forked a child({})", getpid(), child);
      child
    }
    Ok(ForkResult::Child) => {
      println!("Main({}) forked a PPID ({})", getpid(), getppid()); // now you can see PPID!
      exit(0);
    }
    Err(_) => panic!("Fork failed"), // return を期待している時は　panic にする
  };

  match waitpid(child_pid, None) {
    Ok(status) => println!("Child exited({:?})", status),
    Err(_) => println!("waitpid() failed"),
  }
}

pub fn test() {
  wait_child();
}
