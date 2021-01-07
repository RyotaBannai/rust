// Make 1 module in 1 file.
use crate::module_b; //同じパッケージ内のモジュールを使う場合は, crate から始める（絶対パス）

mod module_a {
  fn calc() {
    print!("calculating...")
  }
}
