pub fn test_result() {
  let result: Result<i32, String> = Ok(200);
  match result {
    Ok(code) => println!("code: {}", code),
    Err(err) => println!("Err: {}", err),
  }
}

pub fn test_result2() {
  let result: Result<i32, String> = Ok(200);
  if let Ok(code) = result {
    println!("code: {}", code);
  }
}

/*
  unwrap_or を使うと Ok() だった場合はそのまま展開し、Err() だった場合は引数で与えた値を返す
*/
pub fn test_result3() {
  let result1: Result<i32, String> = Ok(200);
  println!("code: {}", result1.unwrap_or(-1)); // 200

  let result2: Result<i32, String> = Err("error".to_string());
  println!("code: {}", result2.unwrap_or(-1)); // 1
}

// use and_then()
fn func(code: i32) -> Result<i32, String> {
  println!("code: {}", code);
  Ok(100)
}
fn func_main() {
  let result: Result<i32, String> = Ok(200);
  let next_result = result.and_then(func); // func は実行される
}
