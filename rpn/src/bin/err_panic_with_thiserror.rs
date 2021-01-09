use thiserror::Error;

#[derive(Error, Debug)]
enum MyErrors {
  #[error("failed to read string from {0}")]
  ReadError(String),
  #[error(transparent)]
  ParseParse(#[from] std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyErrors> {
  let path = "./src/number.txt";
  let num_str = std::fs::read_to_string(path).map_err(|_| MyErrors::ReadError(path.into()))?;
  let ret = num_str
    .trim()
    .parse::<i32>()
    .map(|t| t * 2)
    .map_err(MyErrors::from); // from で受け取る
  ret
}

fn main() {
  match get_int_from_file() {
    Ok(x) => println!("{}", x),
    Err(e) => println!("{:#?}", e),
  }
}
