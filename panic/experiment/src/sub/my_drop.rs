pub struct Droppable {
  pub name: &'static str,
}

pub struct A(bool);

impl Drop for Droppable {
  // drop 処理中はスタックトレースを巻き戻している最中のため、再度 panic を起こさないようにする（２重パニック => プログラム全体を強制終了(Abort)）
  fn drop(&mut self) {
    println!("> Dropping {}", self.name)
  }
}

impl Drop for A {
  fn drop(&mut self) {
    if self.0 {
      eprintln!("Something happened! Cleaning up...")
    }
  }
}
