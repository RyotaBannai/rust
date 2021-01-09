use std::fmt;

enum MyErrors {
  Io(std::io::Error),
  IntParse(std::num::ParseIntError),
}

impl fmt::Display for MyErrors {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MyErrors::Io(cause) => write!(f, "I/O Error: {}", cause),
      MyErrors::IntParse(cause) => write!(f, "IntParse Error: {}", cause),
    }
  }
}

fn get_int_from_file() -> Result<i32, MyErrors> {
  let path = "./src/number.txt";
  let num_str = std::fs::read_to_string(path)
    // .expect(&format!("failed to open file {}", path))
    .map_err(|e| MyErrors::Io(e))?; // panic を起こす代わりに、Result<> を使って Err を返す
                                    // ? は Result<> の後に使用できる
                                    // Ok(t) であれば t を返し、Err(e) であれば Err(e) を早期リターンして関数を終了
  let ret = num_str
    .trim()
    .parse::<i32>()
    .map(|t| t * 2)
    // .expect(&format!("failed to parse string {}", num_str)) // ParseIntError
    .map_err(|e| MyErrors::IntParse(e));
  ret
}

fn main() {
  match get_int_from_file() {
    Ok(x) => println!("{}", x),
    Err(e) => println!("{}", e),
  }
}
