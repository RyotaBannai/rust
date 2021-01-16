// cargo uses this file when it builds.
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
  // memory.x は対象の CPU がどのようなメモリアドレス空間を持っているか示すファイル
  // 具体的な値はチップごとに異なるため、ベンダーの天共するスペックシートを見ながら設定
  let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap()); // fetches the environment variable key from the current process.
                                                             // OUT_DIR はビルド中の中間ファイルを生成するディレクトリパス
  File::create(out.join("memory.x"))
    .unwrap()
    .write_all(include_bytes!("memory.x")) // memory.x をそこの書き込む. include_bytes! でバイト列を取得
    .unwrap();

  // build.rs の中で println! → cargo へのオプション
  println!("cargo:rustc-link-search={}", out.display()); // set link preference when cargo builds
  println!("cargo:rerun-if-changed=memory.x");
}
